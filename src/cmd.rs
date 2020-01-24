use cargo::core::shell::Shell;
use cargo::core::shell::Verbosity::{Normal, Quiet, Verbose};
use failure::Error;
use std::path::Path;
use std::process::Command;
use termcolor::Color::Green;

///
/// Shell out and execute the specified command. Change to the path first and
/// only execute the command if a dry run has not been requested.
///
pub fn call(command: &[&str], path: &str, shell: &mut Shell, dry_run: bool) -> Result<bool, Error> {
    if !Path::new(path).exists() {
        shell.error(format!(
            "Documentation path '{}' not present. \
             Was this included as argument to `sphinx-generate`?",
            path
        ))?;
        return Ok(false);
    }

    if dry_run {
        shell.status_with_color("", format!("cd {}", path), Green)?;
        shell.status_with_color("", command.join(" "), Green)?;
        shell.status_with_color("", "cd -", Green)?;

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
            let mut child = cmd.spawn()?;
            let result = child.wait()?;
            Ok(result.success())
        }
        Quiet => {
            let output = cmd.output()?;
            if !output.status.success() {
                shell.error(String::from_utf8_lossy(&output.stderr))?;
            }
            Ok(output.status.success())
        }
    }
}
