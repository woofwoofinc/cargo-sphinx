use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

use toml;
use toml::Value;
use toml::value::Table;
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

pub struct Config {
    toml: Table,
}

impl Config {
    fn load_from_file(path: &Path) -> io::Result<String> {
        let mut file = try!(File::open(path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        Ok(contents)
    }

    fn value_to_table(value: Value) -> Option<Table> {
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
    pub fn from(path: &str) -> Result<Config, FatalError> {
        let path = Path::new(path);
        let contents = try!(Config::load_from_file(path).map_err(|e| FatalError::IO(e)));

        let parsed: Option<Table> = toml::from_str(&contents).ok();

        // Verify parsed TOML is valid.
        let mut toml: Table = try!(parsed.ok_or(FatalError::InvalidCargoFileFormat));

        let config: Table = toml.remove("package")
            .and_then(Config::value_to_table)
            .and_then(|mut table| table.remove("metadata"))
            .and_then(Config::value_to_table)
            .and_then(|mut table| table.remove("sphinx"))
            .and_then(Config::value_to_table)
            .unwrap_or_default();

        // Verify the Cargo Sphinx TOML configuration.
        let valid_keys = vec![
            DOCS_PATH,
            COMMIT_MESSAGE,
            SIGN_COMMIT,
            PUSH_REMOTE,
            PUSH_BRANCH,
        ];

        for key in config.keys() {
            if !valid_keys.contains(&key.as_ref()) {
                return Err(FatalError::UnknownCargoFileKey { key: key.to_string() });
            }
        }

        Ok(Config { toml: config })
    }

    ///
    /// Get a string property from this configuration.
    ///
    pub fn get_str(&self, key: &'static str) -> Option<&str> {
        self.toml.get(key).and_then(|value| value.as_str())
    }

    ///
    /// Get a boolean property from this configuration.
    ///
    pub fn get_bool(&self, key: &'static str) -> Option<bool> {
        self.toml.get(key).and_then(|value| value.as_bool())
    }
}

#[test]
fn test_docs_path_config() {
    // Check docs_path is set to "docs" in Cargo.toml of this repository.
    let result: Result<Config, FatalError> = Config::from("Cargo.toml");
    let config: Config = result.expect("Parse cargo file failed.");

    assert_eq!(config.get_str("docs-path"), Some("docs"));
}

#[test]
fn test_commit_message_config() {
    // Check commit_message is set to "(cargo-sphinx) Generate docs." in
    // Cargo.toml of this repository.
    let result: Result<Config, FatalError> = Config::from("Cargo.toml");
    let config: Config = result.expect("Parse cargo file failed.");

    assert_eq!(
        config.get_str("commit-message"),
        Some("(cargo-sphinx) Generate docs.")
    );
}

#[test]
fn test_sign_commit_config() {
    // Check sign-commit is set to false in Cargo.toml of this repository.
    let result: Result<Config, FatalError> = Config::from("Cargo.toml");
    let config: Config = result.expect("Parse cargo file failed.");

    assert_eq!(config.get_bool("sign-commit"), Some(false));
}

#[test]
fn test_push_remote_config() {
    // Check push-remote is set to "origin" in Cargo.toml of this repository.
    let result: Result<Config, FatalError> = Config::from("Cargo.toml");
    let config: Config = result.expect("Parse cargo file failed.");

    assert_eq!(config.get_str("push-remote"), Some("origin"));
}

#[test]
fn test_push_branch_config() {
    // Check push-branch is set to "gh-pages" in Cargo.toml of this repository.
    let result: Result<Config, FatalError> = Config::from("Cargo.toml");
    let config: Config = result.expect("Parse cargo file failed.");

    assert_eq!(config.get_str("push-branch"), Some("gh-pages"));
}
