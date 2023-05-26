use bitreader::Readable;
use other::{Page, Parse};
use v6::{Block, SceneTree, TaggedBitreader, TypeParse};

pub mod bitreader;
pub mod other;
pub mod parse_error;
pub mod shared;
pub mod v6;

pub use crate::parse_error::ParseErrorKind;
pub use bitreader::Bitreader;
pub use parse_error::ParseError;

#[derive(Debug)]
pub enum RemarkableFile {
    V6 { tree: SceneTree, blocks: Vec<Block> },
    Other { version: u32, pages: Vec<Page> },
}

impl RemarkableFile {
    pub fn read(input: impl Readable) -> Result<RemarkableFile, ParseError> {
        let mut reader = Bitreader::new(input);
        return Self::read_impl(&mut reader)
            .map_err(|e| e.with_context_from_bitreader(&mut reader));
    }

    fn read_impl(reader: &mut Bitreader<impl Readable>) -> Result<RemarkableFile, ParseError> {
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

        if version == 6 {
            let mut blocks = vec![];
            let mut tagged_bit_reader = TaggedBitreader::new(reader);

            loop {
                if tagged_bit_reader.bit_reader.eof()? {
                    break;
                }
                blocks.push(Block::parse(&mut tagged_bit_reader)?);
            }

            let tree = SceneTree::from_blocks(&blocks)?;
            return Ok(RemarkableFile::V6 { tree, blocks });
        }

        if 3 > version || version > 6 {
            return Err(ParseError::new(
                &format!("version '{version}' is not supported"),
                ParseErrorKind::Unsupported,
            ));
        }

        let amount_pages = if version >= 3 { 1 } else { reader.read_u32()? };

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
