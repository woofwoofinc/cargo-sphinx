use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::fs::{self, File};
use std::path::Path;

use toml::{Parser, Value, Table};
use semver::Version;
use regex::Regex;

use error::FatalError;

fn load_from_file(path: &Path) -> io::Result<String> {
    let mut file = try!(File::open(path));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    Ok(s)
}

fn save_to_file(path: &Path, content: &str) -> io::Result<()> {
    let mut file = try!(File::create(path));
    try!(file.write_all(&content.as_bytes()));
    Ok(())
}

pub fn parse_cargo_config() -> Result<Table, FatalError> {
    let cargo_file_path = Path::new("Cargo.toml");

    let cargo_file_content = try!(load_from_file(&cargo_file_path)
                                  .map_err(FatalError::from));

    let mut parser = Parser::new(&cargo_file_content);

    match parser.parse() {
        Some(toml) => Ok(toml),
        None => Err(FatalError::InvalidCargoFileFormat)
    }
}

pub fn save_cargo_config(config: &Table) -> Result<(), FatalError> {
    let cargo_file_path = Path::new("Cargo.toml");

    let serialized_data = format!("{}", Value::Table(config.clone()));

    try!(save_to_file(&cargo_file_path, &serialized_data).map_err(FatalError::from));
    Ok(())
}

pub fn rewrite_cargo_version(version: &str) -> Result<(), FatalError> {
    let section_matcher = Regex::new("^\\[.+\\]").unwrap();
    {
        let file_in = try!(File::open("Cargo.toml").map_err(FatalError::from));
        let mut bufreader = BufReader::new(file_in);
        let mut line = String::new();

        let mut file_out = try!(File::create("Cargo.toml.work").map_err(FatalError::from));
        let mut in_package = false;

        loop {
            let b = try!(bufreader.read_line(&mut line).map_err(FatalError::from));
            if b <= 0 {
                break;
            }

            if section_matcher.is_match(&line) {
                in_package = line.trim() == "[package]";
            }

            if in_package && line.starts_with("version") {
                line = format!("version = \"{}\"\n", version);
            }

            try!(file_out.write_all(line.as_bytes()).map_err(FatalError::from));
            line.clear();
        }
    }
    try!(fs::rename("Cargo.toml.work", "Cargo.toml"));

    {
        let file_in = try!(File::open("Cargo.lock").map_err(FatalError::from));
        let mut bufreader = BufReader::new(file_in);
        let mut line = String::new();

        let mut file_out = try!(File::create("Cargo.lock.work").map_err(FatalError::from));
        let mut in_package = false;

        loop {
            let b = try!(bufreader.read_line(&mut line).map_err(FatalError::from));
            if b <= 0 {
                break;
            }

            if section_matcher.is_match(&line) {
                in_package = line.trim() == "[root]";
            }

            if in_package && line.starts_with("version") {
                line = format!("version = \"{}\"\n", version);
            }

            try!(file_out.write_all(line.as_bytes()).map_err(FatalError::from));
            line.clear();
        }
    }

    try!(fs::rename("Cargo.lock.work", "Cargo.lock"));
    Ok(())
}

pub fn parse_version(version: &str) -> Result<Version, FatalError> {
    Version::parse(version).map_err(|e| FatalError::from(e))
}
