use std::process::Command;
use std::env::current_dir;

use error::FatalError;

pub fn call(command: Vec<&str>) -> Result<bool, FatalError> {
    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);

    for arg in iter {
        if arg.len() > 0 {
            cmd.arg(arg);
        }
    }

    let mut child = try!(cmd.spawn().map_err(FatalError::from));
    let result = try!(child.wait().map_err(FatalError::from));

    Ok(result.success())
}

pub fn relative_path_for(root: &str) -> Result<Option<String>, FatalError> {
    let pwd = try!(current_dir()).to_str().unwrap().to_owned();
    if pwd.len() > root.len() {
        Ok(Some(pwd[root.len()..].to_string()))
    } else {
        Ok(None)
    }
}
