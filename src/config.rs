use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

use toml::{Parser, Value, Table};
use error::FatalError;


///
/// `Cargo.toml` key under `package.metadata.sphinx` for specifying a default
/// location for the project Sphinx documentation files.
///
pub static DOCS_PATH: &'static str = "docs-path";

///
/// `Cargo.toml` key under `package.metadata.sphinx` for specifying a default
/// Git commit message for documentation commits.
///
pub static COMMIT_MESSAGE: &'static str = "commit-message";

///
/// `Cargo.toml` key under `package.metadata.sphinx` for specifying a default
/// boolean value for whether to sign documentation commits.
///
pub static SIGN_COMMIT: &'static str = "sign-commit";

///
/// `Cargo.toml` key under `package.metadata.sphinx` for specifying a default
/// Git remote name for pushing documentation commits.
///
pub static PUSH_REMOTE: &'static str = "push-remote";

///
/// `Cargo.toml` key under `package.metadata.sphinx` for specifying a default
/// branch for pushing documentation commits.
///
pub static PUSH_BRANCH: &'static str = "push-branch";

fn load_from_file(path: &Path) -> io::Result<String> {
    let mut file = try!(File::open(path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}

fn as_table(value: Value) -> Option<Table> {
    match value {
        Value::Table(s) => Some(s),
        _ => None,
    }
}

///
/// Parse the `Cargo.toml` file in the current directory and extract the keys
/// under `package.metadata.sphinx`. This contains execution parameter defaults
/// for the project using this cargo plugin.
///
pub fn parse_config() -> Result<Table, FatalError> {
    let cargo_file_path = Path::new("Cargo.toml");
    let cargo_file_content = try!(load_from_file(&cargo_file_path).map_err(FatalError::from));

    let mut parser = Parser::new(&cargo_file_content);

    let toml = parser.parse();
    let package = toml.and_then(|mut table| table.remove("package").and_then(as_table));
    let metadata = package.and_then(|mut table| table.remove("metadata").and_then(as_table));
    let sphinx = metadata.and_then(|mut table| table.remove("sphinx").and_then(as_table));

    sphinx.ok_or(FatalError::InvalidCargoFileFormat)
}

///
/// Get a string property from a `parse_config()` response.
///
pub fn get_str<'a>(table: &'a Table, key: &'static str) -> Option<&'a str> {
    table.get(key).and_then(|value| value.as_str())
}

///
/// Get a boolean property from a `parse_config()` response.
///
pub fn get_bool(table: &Table, key: &'static str) -> Option<bool> {
    table.get(key).and_then(|value| value.as_bool())
}

#[test]
fn test_docs_path_config() {
    // Check docs_path is set to "docs" in Cargo.toml of this repository.
    let config: Result<Table, FatalError> = parse_config();
    let table: Table = config.expect("Parse cargo file failed.");

    assert_eq!(get_str(&table, "docs-path"), Some("docs"));
}

#[test]
fn test_commit_message_config() {
    // Check commit_message is set to "(cargo-sphinx) Generate docs." in
    // Cargo.toml of this repository.
    let config: Result<Table, FatalError> = parse_config();
    let table: Table = config.expect("Parse cargo file failed.");

    assert_eq!(get_str(&table, "commit-message"),
               Some("(cargo-sphinx) Generate docs."));
}

#[test]
fn test_sign_commit_config() {
    // Check sign-commit is set to false in Cargo.toml of this repository.
    let config: Result<Table, FatalError> = parse_config();
    let table: Table = config.expect("Parse cargo file failed.");

    assert_eq!(get_bool(&table, "sign-commit"), Some(false));
}

#[test]
fn test_push_remote_config() {
    // Check push-remote is set to "origin" in Cargo.toml of this repository.
    let config: Result<Table, FatalError> = parse_config();
    let table: Table = config.expect("Parse cargo file failed.");

    assert_eq!(get_str(&table, "push-remote"), Some("origin"));
}

#[test]
fn test_push_branch_config() {
    // Check push-branch is set to "gh-pages" in Cargo.toml of this repository.
    let config: Result<Table, FatalError> = parse_config();
    let table: Table = config.expect("Parse cargo file failed.");

    assert_eq!(get_str(&table, "push-branch"), Some("gh-pages"));
}
