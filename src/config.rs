use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

use toml::{Parser, Value, Table};

use error::FatalError;

pub static SIGN_COMMIT: &'static str = "sign-commit";
pub static PUSH_REMOTE: &'static str = "push-remote";
pub static DOC_BRANCH: &'static str = "doc-branch";
pub static DOC_COMMIT_MESSAGE: &'static str = "doc-commit-message";

fn load_from_file(path: &Path) -> io::Result<String> {
    let mut file = try!(File::open(path));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    Ok(s)
}

pub fn parse_cargo_config() -> Result<Table, FatalError> {
    let cargo_file_path = Path::new("Cargo.toml");

    let cargo_file_content = try!(load_from_file(&cargo_file_path).map_err(FatalError::from));

    let mut parser = Parser::new(&cargo_file_content);

    match parser.parse() {
        Some(toml) => Ok(toml),
        None => Err(FatalError::InvalidCargoFileFormat),
    }
}

pub fn get_release_config<'a>(config: &'a Table, key: &str) -> Option<&'a Value> {
    config.get("package")
        .and_then(|f| f.as_table())
        .and_then(|f| f.get("metadata"))
        .and_then(|f| f.as_table())
        .and_then(|f| f.get("gh-pages"))
        .and_then(|f| f.as_table())
        .and_then(|f| f.get(key))
}

pub fn verify_release_config(config: &Table) -> Option<Vec<&str>> {
    let valid_keys = vec![SIGN_COMMIT, PUSH_REMOTE, DOC_BRANCH, DOC_COMMIT_MESSAGE];
    if let Some(r) = config.get("package")
        .and_then(|f| f.as_table())
        .and_then(|f| f.get("metadata"))
        .and_then(|f| f.as_table())
        .and_then(|f| f.get("gh-pages"))
        .and_then(|f| f.as_table()) {
        let mut invalid_keys = Vec::new();
        for i in r.keys() {
            if !valid_keys.contains(&i.as_ref()) {
                invalid_keys.push(i.as_ref());
            }
        }
        if invalid_keys.is_empty() {
            None
        } else {
            Some(invalid_keys)
        }
    } else {
        None
    }
}

#[test]
fn test_release_config() {
    if let Ok(cargo_file) = parse_cargo_config() {
        // Check sign-commit set to false in Cargo.toml of this repository.
        assert!(get_release_config(&cargo_file, "sign-commit")
            .and_then(|f| f.as_bool())
            .map(|f| f == false)
            .unwrap_or(false));
    } else {
        panic!("Parse cargo file failed.");
    }
}
