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

pub fn remote_get_url(remote: &str) -> Result<String, FatalError> {
    let output = try!(Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg(remote)
        .output()
        .map_err(FatalError::from));
    String::from_utf8(output.stdout).map_err(FatalError::from)
}

pub fn init(dir: &str, dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git", "init"], dir, dry_run)
}

pub fn add_all(dir: &str, dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git", "add", "."], dir, dry_run)
}

pub fn commit_all(dir: &str, msg: &str, sign: bool, dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git", "commit", if sign { "-S" } else { "" }, "-am", msg],
         dir,
         dry_run)
}

pub fn force_push(dir: &str,
                  remote: &str,
                  refspec: &str,
                  dry_run: bool)
                  -> Result<bool, FatalError> {
    call(vec!["git", "push", "-f", remote, refspec], dir, dry_run)
}
