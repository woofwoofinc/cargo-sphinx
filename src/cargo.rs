use cmd::call;
use error::FatalError;

pub fn doc(dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["cargo", "doc", "--no-deps"], dry_run)
}
