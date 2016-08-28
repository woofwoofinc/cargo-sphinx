use std::io::Error as IOError;
use std::string::FromUtf8Error;
use cargo::CargoError;

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
        UnknownCargoFileKey {
            display("Unknown cargo key found")
            description("Unknown config key found")
        }
        CargoError(err: Box<CargoError>) {
            from()
            cause(err)
        }
        FromUtf8Error(err: FromUtf8Error) {
            from()
            cause(err)
        }
    }
}
