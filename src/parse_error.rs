use std::{
    error, fmt, i32,
    io::{self, Read},
    num, string, u32,
};

use super::Bitreader;

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
    context: String,
}

impl ParseError {
    pub fn new<S: Into<String>>(message: S, kind: ParseErrorKind) -> ParseError {
        ParseError {
            message: message.into(),
            kind,
            context: String::new(),
        }
    }

    pub fn invalid<S: Into<String>>(message: S) -> ParseError {
        Self::new(message, ParseErrorKind::InvalidInput)
    }

    pub fn unsupported<S: Into<String>>(message: S) -> ParseError {
        Self::new(message, ParseErrorKind::Unsupported)
    }

    pub fn with_bitreader(mut self, bitreader: &Bitreader<impl Read>) -> ParseError {
        self.context = format!(
            "error occured while parsing at bit position: {:x}",
            bitreader.offset()
        );
        self
    }
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error while parsing remarkable file {}. {}",
            self.message,
            format!("\n{}", self.context)
        )
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
            format!("Failed to parse integer: {error}"),
            ParseErrorKind::InvalidInput,
        )
    }
}
impl From<num::TryFromIntError> for ParseError {
    fn from(error: num::TryFromIntError) -> Self {
        Self::new(
            format!("Failed to read input: {error}"),
            ParseErrorKind::InvalidInput,
        )
    }
}

impl From<string::FromUtf8Error> for ParseError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::invalid(format!("Invalid utf-8 string found: '{value}'"))
    }
}
