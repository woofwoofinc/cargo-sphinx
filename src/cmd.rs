use std::process::Command;
use error::FatalError;
use cargo::core::MultiShell;
use cargo::core::shell::Verbosity::{Verbose, Normal, Quiet};
use term::color;

///
/// Shell out and execute the specified command. Change to the path first and
/// only execute the command if a dry run has not been requested.
///
pub fn call(command: Vec<&str>,
            path: &str,
            shell: &mut MultiShell,
            dry_run: bool)
            -> Result<bool, FatalError> {
    if dry_run {
        try!(shell.say(format!("cd {}", path), color::GREEN));
        try!(shell.say(format!("{}", command.join(" ")), color::GREEN));
        try!(shell.say("cd -", color::GREEN));

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

    match shell.get_verbose() {
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
