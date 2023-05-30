use std::{
    error, fmt,
    io::{self},
    num::TryFromIntError,
    string,
};

use crate::bitreader::Readable;

use super::Bitreader;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    Io,
    InvalidInput,
    Unsupported,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub context: String,
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

    /// Add extra context from the bitreader used to parse something
    pub fn with_context_from_bitreader(
        mut self,
        bitreader: &mut Bitreader<impl Readable>,
    ) -> ParseError {
        match bitreader.eof() {
            Ok(eof) => {
                if eof {
                    self.context = String::from("error occured after data has been read.");
                } else {
                    self.context = format!(
                        "error occured while parsing at bit position: {:x}",
                        bitreader.position()
                    );
                }
                self
            }
            Err(e) => Self::invalid(format!(
                "Failed to read eof of bitreader when trying to add context: {}",
                e
            )),
        }
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
// impl From<num::ParseIntError> for ParseError {
//     fn from(error: num::ParseIntError) -> Self {
//         Self::new(
//             format!("Failed to parse integer: {error}"),
//             ParseErrorKind::InvalidInput,
//         )
//     }
// }
impl From<TryFromIntError> for ParseError {
    fn from(error: TryFromIntError) -> Self {
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
