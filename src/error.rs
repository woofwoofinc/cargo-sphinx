use std::io::{Error as IOError};
use semver::SemVerError;

quick_error! {
    #[derive(Debug)]
    pub enum FatalError {
        IOError(err: IOError) {
            from()
            cause(err)
        }
        InvalidCargoFileFormat {
            display("Invalid cargo file format")
            description("Invalid cargo file format")
        }
        SemVerError(err: SemVerError) {
            from()
            cause(err)
        }
    }
}
