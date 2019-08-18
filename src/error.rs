use std::{error, fmt};

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }

    pub fn to_string(self) -> String {
        self.into()
    }
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.message.fmt(formatter)
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        e.message
    }
}
