use std::process::{Command};

use error::FatalError;

pub fn call(command: Vec<&str>) -> Result<bool, FatalError> {
    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);

    for arg in iter {
        cmd.arg(arg);
    }

    let mut child = try!(cmd.spawn().map_err(FatalError::from));
    let result = try!(child.wait().map_err(FatalError::from));

    Ok(result.success())
}
