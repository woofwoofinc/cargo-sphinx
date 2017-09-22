use std::process::Command;
use error::FatalError;
use cargo::core::shell::Shell;
use cargo::core::shell::Verbosity::{Verbose, Normal, Quiet};
use termcolor::Color::Green;

///
/// Shell out and execute the specified command. Change to the path first and
/// only execute the command if a dry run has not been requested.
///
pub fn call(
    command: Vec<&str>,
    path: &str,
    shell: &mut Shell,
    dry_run: bool,
) -> Result<bool, FatalError> {
    if dry_run {
        try!(shell.status_with_color("", format!("cd {}", path), Green));
        try!(shell.status_with_color(
            "",
            format!("{}", command.join(" ")),
            Green,
        ));
        try!(shell.status_with_color("", "cd -", Green));

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
                try!(shell.error(String::from_utf8_lossy(&output.stderr)));
            }
            Ok(output.status.success())
        }
    }
}
