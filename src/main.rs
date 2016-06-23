#![allow(dead_code)]

#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate toml;
extern crate semver;
extern crate clap;

use std::process::exit;

use clap::{App, ArgMatches, SubCommand};
use semver::Identifier;

mod config;
mod error;
mod cmd;
mod git;
mod cargo;

fn execute(args: &ArgMatches) -> Result<i32, error::FatalError> {
    let cargo_file = try!(config::parse_cargo_config());

    let dry_run = args.occurrences_of("dry-run") > 0;
    let sign = args.occurrences_of("sign") > 0 ||
               config::get_release_config(&cargo_file, "sign-tag")
                   .and_then(|f| f.as_bool())
                   .unwrap_or(false);
    let upload_doc = args.occurrences_of("upload-doc") > 0 ||
                     config::get_release_config(&cargo_file, "upload-doc")
                         .and_then(|f| f.as_bool())
                         .unwrap_or(false);
    let git_remote = args.value_of("push-remote")
                         .or_else(|| {
                             config::get_release_config(&cargo_file, "push-remote")
                                 .and_then(|f| f.as_str())
                         })
                         .unwrap_or("origin");
    let doc_branch = args.value_of("doc-branch")
                         .or_else(|| {
                             config::get_release_config(&cargo_file, "doc-branch")
                                 .and_then(|f| f.as_str())
                         })
                         .unwrap_or("gh-pages");
    let skip_push = args.occurrences_of("skip-push") > 0 ||
                    config::get_release_config(&cargo_file, "disable-push")
                        .and_then(|f| f.as_bool())
                        .unwrap_or(false);

    let pre_release_commit_msg = config::get_release_config(&cargo_file,
                                                            "pre-release-commit-message")
                                     .and_then(|f| f.as_str())
                                     .unwrap_or("(cargo-release) version {}");
    let pro_release_commit_msg =
        config::get_release_config(&cargo_file, "pro-release-commit-message")
            .and_then(|f| f.as_str())
            .unwrap_or("(cargo-release) start next development iteration {}");
    let tag_msg = config::get_release_config(&cargo_file, "tag-message")
                      .and_then(|f| f.as_str())
                      .unwrap_or("(cargo-release) {} version {}");
    let doc_commit_msg = config::get_release_config(&cargo_file, "doc-commit-message")
                             .and_then(|f| f.as_str())
                             .unwrap_or("(cargo-release) generate docs");

    // STEP 0: Check if working directory is clean
    if !try!(git::status()) {
        println!("Uncommitted changes detected, please commit before release");
        if !dry_run {
            return Ok(101);
        }
    }

    // STEP 1: Read version from Cargo.toml and remove
    let mut version = cargo_file.get("package")
                                .and_then(|f| f.as_table())
                                .and_then(|f| f.get("version"))
                                .and_then(|f| f.as_str())
                                .and_then(|f| config::parse_version(f).ok())
                                .unwrap();

    // STEP 2: update current version, save and commit
    let mut need_commit = false;
    match args.value_of("level") {
        Some(level) => {
            match level {
                "major" => {
                    version.increment_major();
                    need_commit = true;
                }
                "minor" => {
                    version.increment_minor();
                    need_commit = true
                }
                "patch" => {
                    if !version.is_prerelease() {
                        version.increment_patch();
                    } else {
                        version.pre.clear();
                    }
                    need_commit = true
                }
                _ => {
                    panic!("Invalid level: {}", level);
                }
            }
        }
        None => {
            if version.is_prerelease() {
                version.pre.clear();
                need_commit = true;
            }
        }
    }

    if need_commit {
        let new_version_string = version.to_string();
        if !dry_run {
            try!(config::rewrite_cargo_version(&new_version_string));
        }

        let commit_msg = String::from(pre_release_commit_msg)
                             .replace("{{version}}", &new_version_string);
        if !try!(git::commit_all(".", &commit_msg, sign, dry_run)) {
            // commit failed, abort release
            return Ok(102);
        }
    }

    // STEP 3: cargo publish
    if !try!(cargo::publish(dry_run)) {
        return Ok(103);
    }

    // STEP 4: upload doc
    if upload_doc {
        println!("Building and exporting docs.");
        try!(cargo::doc(dry_run));

        let doc_path = "target/doc/";

        try!(git::init(doc_path, dry_run));
        try!(git::add_all(doc_path, dry_run));
        try!(git::commit_all(doc_path, doc_commit_msg, sign, dry_run));
        let default_remote = try!(git::origin_url());

        let mut refspec = String::from("master:");
        refspec.push_str(doc_branch);

        try!(git::force_push(doc_path, default_remote.trim(), &refspec, dry_run));
    }


    // STEP 5: Tag
    let root = try!(git::top_level());
    let rel_path = try!(cmd::relative_path_for(&root));
    let tag_prefix = args.value_of("tag-prefix")
                         .map(|t| t.to_owned())
                         .or(rel_path.as_ref().map(|t| format!("{}-", t)));

    let current_version = version.to_string();
    let tag_name = tag_prefix.as_ref().map_or_else(|| current_version.clone(),
                                                   |x| format!("{}{}", x, current_version));

    let tag_message = String::from(tag_msg)
                          .replace("{{module}}", &rel_path.unwrap_or("".to_owned()))
                          .replace("{{version}}", &current_version);

    if !try!(git::tag(&tag_name, &tag_message, sign, dry_run)) {
        // tag failed, abort release
        return Ok(104);
    }

    // STEP 6: bump version
    version.increment_patch();
    version.pre.push(Identifier::AlphaNumeric("pre".to_owned()));
    println!("Starting next development iteration {}", version);
    let updated_version_string = version.to_string();
    if !dry_run {
        try!(config::rewrite_cargo_version(&updated_version_string));
    }
    let commit_msg = String::from(pro_release_commit_msg)
                         .replace("{{version}}", &updated_version_string);

    if !try!(git::commit_all(".", &commit_msg, sign, dry_run)) {
        return Ok(105);
    }

    // STEP 7: git push
    if !skip_push {
        if !try!(git::push(git_remote, dry_run)) {
            return Ok(106);
        }
    }

    Ok(0)
}

static USAGE: &'static str = "-l, --level=[level] 'Release level: bumpping major|minor|patch version on release or removing prerelease extensions by default'
                             [sign]... --sign 'Sign git commit and tag'
                             [dry-run]... --dry-run 'Do not actually change anything.'
                             [upload-doc]... --upload-doc 'Upload rust document to gh-pages branch'
                             --push-remote=[push-remote] 'Git remote to push'
                             [skip-push]... --skip-push 'Do not run git push in the last step'
                             --doc-branch=[doc-branch] 'Git branch to push documentation on'
                             --tag-prefix=[tag-prefix] 'Prefix of git tag, note that this will override default prefix based on sub-directory ";

fn main() {
    let matches =
        App::new("cargo")
            .subcommand(SubCommand::with_name("release")
                            .version(env!("CARGO_PKG_VERSION"))
                            .author("Ning Sun <sunng@about.me>")
                            .about("Cargo subcommand for you to smooth your release process.")
                            .args_from_usage(USAGE))
            .get_matches();

    if let Some(ref release_matches) = matches.subcommand_matches("release") {
        match execute(release_matches) {
            Ok(code) => exit(code),
            Err(e) => {
                println!("Fatal: {}", e);
                exit(128);
            }
        }
    }
}
