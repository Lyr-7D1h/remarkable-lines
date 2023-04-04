use std::{error, fmt, io, num};

#[derive(Debug)]
pub enum ParseErrorKind {
    Io,
    InvalidInput,
    Unsupported,
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    message: String,
}

impl ParseError {
    pub fn new<S: Into<String>>(message: S, kind: ParseErrorKind) -> ParseError {
        ParseError {
            message: message.into(),
            kind,
        }
    }

    pub fn invalid<S: Into<String>>(message: S) -> ParseError {
        Self::new(message, ParseErrorKind::InvalidInput)
    }

    pub fn unsupported<S: Into<String>>(message: S) -> ParseError {
        Self::new(message, ParseErrorKind::Unsupported)
    }
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while parsing remarkable file {}", self.message)
    }
}
impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        Self::new(format!("Failed to read input: {error}"), ParseErrorKind::Io)
    }
}
impl From<num::ParseIntError> for ParseError {
    fn from(error: num::ParseIntError) -> Self {
        Self::new(
            format!("Failed to pares integer: {error}"),
            ParseErrorKind::InvalidInput,
        )
    }
}
