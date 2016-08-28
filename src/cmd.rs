use std::process::Command;
use error::FatalError;

///
/// Shell out and execute the specified command. Change to the path first and
/// only execute the command if a dry run has not been requested.
///
pub fn call(command: Vec<&str>, path: &str, dry_run: bool) -> Result<bool, FatalError> {
    if dry_run {
        println!("cd {}", path);
        println!("{}", command.join(" "));
        println!("cd -");

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

    let mut child = try!(cmd.spawn().map_err(FatalError::from));
    let result = try!(child.wait().map_err(FatalError::from));

    Ok(result.success())
}
