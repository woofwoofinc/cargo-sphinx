use cmd::call;
use error::FatalError;

pub fn publish() -> Result<bool, FatalError> {
    call(vec!["cargo", "publish"])
}

pub fn update() -> Result<bool, FatalError> {
    call(vec!["cargo", "update"])
}
