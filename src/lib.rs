use std::io::Read;

use other::page::Page;
use v6::SceneTree;

pub mod bitreader;
pub mod other;
pub mod parse_error;
pub mod v6;

pub use crate::parse_error::ParseErrorKind;
pub use bitreader::Bitreader;
pub use parse_error::ParseError;

pub enum RemarkableFile {
    V6 { tree: SceneTree },
    Other { version: u32, pages: Vec<Page> },
}

pub trait Parse {
    fn parse<N: Read>(version: u32, reader: &mut Bitreader<N>) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl RemarkableFile {
    pub fn read(input: impl Read) -> Result<RemarkableFile, ParseError> {
        let mut reader = Bitreader::new(input);
        return Self::read_impl(&mut reader).map_err(|e| e.with_bitreader(&reader));
    }

    fn read_impl(reader: &mut Bitreader<impl Read>) -> Result<RemarkableFile, ParseError> {
        let version_description = reader
            .read_bytes(43)?
            .into_iter()
            .map(|i| i as char)
            .collect::<String>();

        let version_description = version_description.trim_end();
        let version: u32 = {
            if version_description == "reMarkable lines with selections and layers" {
                // early version of the format that is not supported
                return Err(ParseError::unsupported("Unsupported outdated version"));
            } else if version_description.starts_with("reMarkable .lines file, version=") {
                match version_description.split("=").nth(1) {
                    Some(v) => v.parse().map_err(|_| {
                        ParseError::unsupported(format!(
                            "Could not find version from: {version_description}"
                        ))
                    })?,
                    None => {
                        return Err(ParseError::unsupported(format!(
                            "Unknown version from: {version_description}"
                        )))
                    }
                }
            } else {
                return Err(ParseError::unsupported(format!(
                    "Unknown version from: {version_description}"
                )));
            }
        };

        let amount_pages = if version >= 3 { 1 } else { reader.read_u32()? };

        if version == 6 {
            let tree = SceneTree::parse(version, reader)?;
            return Ok(RemarkableFile::V6 { tree });
        }

        if 3 > version || version > 6 {
            return Err(ParseError::new(
                &format!("version '{version}' is not supported"),
                ParseErrorKind::Unsupported,
            ));
        }

        Ok(RemarkableFile::Other {
            version,
            pages: (0..amount_pages)
                .map(|_| Page::parse(version, reader))
                .collect::<Result<Vec<Page>, ParseError>>()?,
        })
    }

    pub fn version(&self) -> u32 {
        match self {
            RemarkableFile::V6 { .. } => 6,
            RemarkableFile::Other { version, .. } => *version,
        }
    }
}
