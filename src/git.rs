use std::process::Command;

use cmd::call;
use error::FatalError;

pub fn status() -> Result<bool, FatalError> {
    let output = try!(Command::new("git")
                          .arg("diff")
                          .arg("--exit-code")
                          .output()
                          .map_err(FatalError::from));
    Ok(output.status.success())
}

pub fn commit_all(msg: &str, sign: bool, dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git",
              "commit",
              if sign {
                  "-S"
              } else {
                  ""
              },
              "-am",
              msg],
         dry_run)
}

pub fn tag(name: &str, msg: &str, sign: bool, dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git",
              "tag",
              "-a",
              name,
              "-m",
              msg,
              if sign {
                  "-s"
              } else {
                  ""
              }],
         dry_run)
}

pub fn push(dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git", "push", "--follow-tags"], dry_run)
}

pub fn top_level() -> Result<String, FatalError> {
    let output = try!(Command::new("git")
                          .arg("rev-parse")
                          .arg("--show-toplevel")
                          .output()
                          .map_err(FatalError::from));
    String::from_utf8(output.stdout).map_err(FatalError::from)
}
