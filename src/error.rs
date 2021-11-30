#[derive(Debug)]
pub enum Error {
    ParseIntError,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        Error::ParseIntError
    }
}
