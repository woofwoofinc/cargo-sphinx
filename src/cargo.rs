use cmd::call;
use error::FatalError;

pub fn publish(dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["cargo", "publish"], dry_run)
}

pub fn update(dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["cargo", "update"], dry_run)
}

pub fn doc(dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["cargo", "doc", "--no-deps"], dry_run)
}
