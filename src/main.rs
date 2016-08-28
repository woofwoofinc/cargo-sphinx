#[macro_use]
extern crate quick_error;
extern crate toml;
extern crate clap;

use std::process::exit;

use toml::Table;
use clap::{App, ArgMatches, SubCommand};

mod config;
mod error;
mod cmd;
mod git;

use cmd::call;

fn execute(args: &ArgMatches) -> Result<i32, error::FatalError> {
    let properties: Table = try!(config::parse_config());

    // Verify the TOML configuration.
    let valid_keys = vec![config::DOCS_PATH,
                          config::COMMIT_MESSAGE,
                          config::SIGN_COMMIT,
                          config::PUSH_REMOTE,
                          config::PUSH_BRANCH];

    for key in properties.keys() {
        if !valid_keys.contains(&key.as_ref()) {
            println!("Unknown config key \"{}\" found for [package.metadata.sphinx]",
                     key);
            return Ok(109);
        }
    }

    // Find parameters or use defaults.
    let dry_run = args.is_present("dry-run");

    let docs_path = args.value_of("docs-path")
        .or_else(|| config::get_str(&properties, config::DOCS_PATH))
        .unwrap_or("docs");

    let push = args.is_present("push");

    let commit_msg = args.value_of("commit-message")
        .or_else(|| config::get_str(&properties, config::COMMIT_MESSAGE))
        .unwrap_or("(cargo-sphinx) Generate docs.");
    let sign = args.is_present("sign") ||
               config::get_bool(&properties, config::SIGN_COMMIT).unwrap_or(false);;

    let push_remote = args.value_of("push-remote")
        .or_else(|| config::get_str(&properties, config::PUSH_REMOTE))
        .unwrap_or("origin");
    let push_branch = args.value_of("push-branch")
        .or_else(|| config::get_str(&properties, config::PUSH_BRANCH))
        .unwrap_or("gh-pages");

    // Check if working directory is clean.
    if !try!(git::status()) {
        println!("Uncommitted changes detected, please commit before release");
        if !dry_run {
            return Ok(101);
        }
    }

    // Generate and upload documentation.
    println!("Building Sphinx docs.");
    try!(call(vec!["make", "clean", "html"], docs_path, dry_run));

    if push {
        println!("Publishing Sphinx docs to GitHub Pages.");
        let docs_build_path = format!("{}/_build", docs_path);

        // A `.nojekyll` file prevents Github from ignoring Sphinx CSS files.
        try!(call(vec!["touch", ".nojekyll"], &docs_build_path, dry_run));

        try!(git::init(&docs_build_path, dry_run));
        try!(git::add_all(&docs_build_path, dry_run));
        try!(git::commit_all(&docs_build_path, commit_msg, sign, dry_run));
        let remote = try!(git::remote_get_url(push_remote));

        let mut refspec = String::from("master:");
        refspec.push_str(push_branch);

        try!(git::force_push(docs_path, remote.trim(), &refspec, dry_run));

        // Clean up.
        try!(call(vec!["rm", "-fr", ".nojekyll", ".git"],
                  &docs_build_path,
                  dry_run));
    }

    Ok(0)
}

fn main() {
    let matches = App::new("cargo")
        .subcommand(SubCommand::with_name("sphinx")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Ning Sun <sunng@about.me>")
            .author("Woof Woof, Inc.")
            .about("Cargo subcommand for generating and publishing Sphinx documentation to \
                    GitHub Pages.")
            .arg_from_usage("--dry-run 'Print commands to execute instead of running'")
            .arg_from_usage("-p, --push 'Push generated documentation to git remote'")
            .arg_from_usage("-s, --sign 'Sign git commit'")
            .arg_from_usage("--docs-path=[docs-path] 'Path of Sphinx documentation to build'")
            .arg_from_usage("--commit-message=[commit-message] 'Commit message for \
                             documentation change'")
            .arg_from_usage("--push-remote=[push-remote] 'Git remote to push'")
            .arg_from_usage("--push-branch=[push-branch] 'Git branch to push documentation on'"))
        .get_matches();

    if let Some(sphinx_matches) = matches.subcommand_matches("sphinx") {
        match execute(sphinx_matches) {
            Ok(code) => exit(code),
            Err(e) => {
                println!("Fatal: {}", e);
                exit(128);
            }
        }
    }
}
