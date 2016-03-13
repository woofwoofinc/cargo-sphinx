#![allow(dead_code)]

#[macro_use] extern crate quick_error;
extern crate regex;
extern crate toml;
extern crate semver;
extern crate clap;

use std::process::exit;

use clap::{App, ArgMatches};
use semver::Identifier;

mod config;
mod error;
mod cmd;
mod git;
mod cargo;

fn execute(args: &ArgMatches) -> Result<i32, error::FatalError> {
    let dry_run = args.occurrences_of("dry-run") > 0;

    // STEP 0: Check if working directory is clean
    if !try!(git::status()) {
        println!("Uncommitted changes detected, please commit before release");
        if !dry_run {
            return Ok(101);
        }
    }

    // STEP 1: Read version from Cargo.toml and remove
    let result = try!(config::parse_cargo_config());

    let mut version = result.get("package")
        .and_then(|f| f.as_table())
        .and_then(|f| f.get("version"))
        .and_then(|f| f.as_str())
        .and_then(|f| config::parse_version(f).ok())
        .unwrap();

    // STEP 2: Remove pre extension, save and commit
    match args.value_of("level") {
        Some(level) => {
            match level {
                "major" => {
                    version.increment_major();
                },
                "minor" => {
                    version.increment_minor();
                },
                "patch" => {
                    version.increment_patch();
                },
                _ => {
                    panic!("Invalid level: {}", level);
                }
            }
        },
        None => {
            version.pre.clear();
        }
    }

    //FIXME: support version bump level
    if version.is_prerelease() {
        version.pre.clear();
        let new_version_string = version.to_string();
        if !dry_run {
            try!(config::rewrite_cargo_version(&new_version_string));
            // sync Cargo.toml and Cargo.lock
            try!(cargo::update());
        }

        let commit_msg = format!("(cargo-release) version {}", new_version_string);
        if !dry_run {
            if !try!(git::commit_all(&commit_msg)) {
                // commit failed, abort release
                return Ok(102);
            }
        }
    }

    // STEP 3: cargo publish
    if !dry_run {
        if !try!(cargo::publish()) {
            return Ok(103);
        }
    }

    // STEP 4: Tag
    let current_version = version.to_string();
    let tag_message = format!("(cargo-release) version {}", current_version);
    if !dry_run {
        if !try!(git::tag(&current_version, &tag_message)) {
            // tag failed, abort release
            return Ok(104);
        }
    }

    // STEP 5: bump version
    version.increment_patch();
    version.pre.push(Identifier::AlphaNumeric("pre".to_owned()));
    println!("Starting next development cycle {}", version);
    let updated_version_string = version.to_string();
    if !dry_run {
        try!(config::rewrite_cargo_version(&updated_version_string));
        try!(cargo::update());
    }
    let commit_msg = format!("(cargo-release) start next development cycle {}",
                             updated_version_string);
    if !dry_run {
        if !try!(git::commit_all(&commit_msg)) {
            return Ok(105);
        }
    }

    // STEP 6: git push
    if !dry_run {
        if !try!(git::push()) {
            return Ok(106);
        }
    }

    Ok(0)
}

fn main() {
    let matches = App::new("cargo release")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ning Sun <sunng@about.me>")
        .about("Cargo subcommand for you to smooth your release process.")
        .args_from_usage("
        -l=[level] 'Release level: bumpping major|minor|patch version on release or removing prerelease extensions by default'
        [dry-run]... --dry-run 'Donot actually change anything.'")
        .get_matches();

    match execute(&matches) {
        Ok(code) => exit(code),
        Err(e) => {
            println!("Fatal: {}", e);
            exit(128);
        }
    }
}
