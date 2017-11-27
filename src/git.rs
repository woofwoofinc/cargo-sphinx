use std::process::Command;

use cargo::core::shell::Shell;
use cmd::call;
use failure::Error;

pub fn remote_get_url(remote: &str) -> Result<String, Error> {
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg(remote)
        .output()?;

    let url = String::from_utf8(output.stdout)?;
    Ok(url)
}

pub fn init(dir: &str, shell: &mut Shell, dry_run: bool) -> Result<bool, Error> {
    call(&["git", "init"], dir, shell, dry_run)
}

pub fn add_all(dir: &str, shell: &mut Shell, dry_run: bool) -> Result<bool, Error> {
    call(&["git", "add", "."], dir, shell, dry_run)
}

pub fn commit_all(
    dir: &str,
    msg: &str,
    sign: bool,
    shell: &mut Shell,
    dry_run: bool,
) -> Result<bool, Error> {
    call(
        &["git", "commit", if sign { "-S" } else { "" }, "-am", msg],
        dir,
        shell,
        dry_run,
    )
}

pub fn force_push(
    dir: &str,
    remote: &str,
    refspec: &str,
    shell: &mut Shell,
    dry_run: bool,
) -> Result<bool, Error> {
    call(&["git", "push", "-f", remote, refspec], dir, shell, dry_run)
}
