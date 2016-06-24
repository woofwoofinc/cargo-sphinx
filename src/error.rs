use std::io::Error as IOError;
use std::string::FromUtf8Error;
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
        InvalidCargoConfigKeys {
            display("Invalid cargo-release config item found")
            description("Invalid cargo-release config item found")
        }
        SemVerError(err: SemVerError) {
            from()
            cause(err)
        }
        FromUtf8Error(err: FromUtf8Error) {
            from()
            cause(err)
        }
    }
}
