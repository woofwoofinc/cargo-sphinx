use cargo::core::shell::Shell;
use cargo::core::shell::Verbosity::{Normal, Quiet, Verbose};
use failure::{Error, SyncFailure};
use std::process::Command;
use termcolor::Color::Green;

///
/// Shell out and execute the specified command. Change to the path first and
/// only execute the command if a dry run has not been requested.
///
pub fn call(command: &[&str], path: &str, shell: &mut Shell, dry_run: bool) -> Result<bool, Error> {
    if dry_run {
        try!(
            shell
                .status_with_color("", format!("cd {}", path), Green)
                .map_err(SyncFailure::new)
        );
        try!(
            shell
                .status_with_color("", format!("{}", command.join(" ")), Green)
                .map_err(SyncFailure::new)
        );
        try!(
            shell
                .status_with_color("", "cd -", Green)
                .map_err(SyncFailure::new)
        );

        return Ok(true);
    }

    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);
    cmd.current_dir(path);

    for arg in iter {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }

    match shell.verbosity() {
        Verbose | Normal => {
            let mut child = try!(cmd.spawn());
            let result = try!(child.wait());
            Ok(result.success())
        }
        Quiet => {
            let output = try!(cmd.output());
            if !output.status.success() {
                try!(
                    shell
                        .error(String::from_utf8_lossy(&output.stderr))
                        .map_err(SyncFailure::new)
                );
            }
            Ok(output.status.success())
        }
    }
}
