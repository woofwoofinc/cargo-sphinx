use std::io::Error as IOError;

#[derive(Fail, Debug)]
pub enum FatalError {
    #[fail(display = "{}", _0)] IO(#[cause] IOError),

    #[fail(display = "Invalid cargo file format.")] InvalidCargoFileFormat,

    #[fail(display = "Unknown cargo key found: {}", key)] UnknownCargoFileKey { key: String },
}
