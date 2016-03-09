#![allow(dead_code)]

#[macro_use] extern crate quick_error;
extern crate regex;
extern crate toml;
extern crate semver;

use std::process::exit;

use semver::Identifier;

mod config;
mod error;
mod cmd;
mod git;
mod cargo;

fn execute() -> Result<i32, error::FatalError> {

    // STEP 0: Check if working directory is clean
    let git_clean = try!(git::status());
    if git_clean {
        println!("Working directory is clean");
    } else {
        println!("Uncommitted changes detected, please commit before release");
        return Ok(128);
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
    if version.is_prerelease() {
        version.pre.clear();
        let new_version_string = version.to_string();
        try!(config::rewrite_cargo_version(&new_version_string));

        let commit_msg = format!("(cargo-release) version {}", new_version_string);
        if !try!(git::commit_all(&commit_msg)) {
            // commit failed, abort release
            return Ok(128);
        }
    }

    // STEP 3: cargo publish
    if !try!(cargo::publish()) {
        return Ok(128);
    }

    // STEP 4: Tag
    let current_version = version.to_string();
    if !try!(git::tag(&current_version)) {
        // tag failed, abort release
        return Ok(128);
    }

    // STEP 5: bump version
    version.increment_patch();
    version.pre.push(Identifier::AlphaNumeric("pre".to_owned()));
    println!("Starting next development cycle {}", version);
    let updated_version_string = version.to_string();
    try!(config::rewrite_cargo_version(&updated_version_string));
    let commit_msg = format!("(cargo-release) start next development cycle {}",
                             updated_version_string);
    if !try!(git::commit_all(&commit_msg)) {
        return Ok(128);
    }

    // STEP 6: git push
    if !try!(git::push()) {
        return Ok(128);
    }
    Ok(0)
}

fn main() {
    match execute() {
        Ok(code) => exit(code),
        Err(e) => {
            println!("Fatal: {}", e);
            exit(128);
        }
    }
}
