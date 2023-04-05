use std::io::Read;

use layer::Layer;
use line::Line;
use page::Page;

pub mod bitreader;
pub mod color;
pub mod layer;
pub mod line;
pub mod page;
pub mod parse_error;
pub mod point;
pub mod tool;

pub use crate::parse_error::ParseErrorKind;
pub use bitreader::Bitreader;
pub use color::Color;
pub use parse_error::ParseError;
pub use point::Point;
pub use tool::Tool;

#[derive(Debug)]
pub struct RemarkableFile {
    pub version: u32,
    pub pages: Vec<Page>,
}

pub trait Parse {
    fn parse<N: Read>(reader: Bitreader<N>) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl RemarkableFile {
    pub fn read(input: impl Read) -> Result<RemarkableFile, ParseError> {
        RemarkableFile::parse(Bitreader::new(input))
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn pages(self) -> Vec<Page> {
        self.pages
    }
}

impl Parse for RemarkableFile {
    /// Parse a remarkable file in little endian order
    fn parse<N: Read>(mut reader: Bitreader<N>) -> Result<Self, ParseError> {
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

        if 3 > version || version > 6 {
            return Err(ParseError::new(
                &format!("version '{version}' is not supported"),
                ParseErrorKind::Unsupported,
            ));
        }

        let amount_pages = if version >= 3 { 1 } else { reader.read_u32()? };

        let pages = (0..amount_pages)
            .map(|_| {
                let amount_layers = reader.read_u32()?;
                let layers = (0..amount_layers)
                    .map(|_| {
                        let amount_lines = reader.read_u32()?;
                        let lines = (0..amount_lines)
                            .map(|_| {
                                let tool = Tool::try_from(reader.read_u32()?)?;
                                let color = Color::try_from(reader.read_u32()?)?;
                                reader.read_u32()?; // Skip unknown value
                                let brush_size = reader.read_f32()?;
                                if version >= 5 {
                                    reader.read_u32()?; // Skip unkown value
                                }
                                let amount_points = reader.read_u32()?;

                                Ok(Line {
                                    tool,
                                    color,
                                    brush_size,
                                    points: (0..amount_points)
                                        .map(|_| {
                                            Ok(Point {
                                                x: reader.read_f32()?,
                                                y: reader.read_f32()?,
                                                speed: reader.read_f32()?,
                                                direction: reader.read_f32()?,
                                                width: reader.read_f32()?,
                                                pressure: reader.read_f32()?,
                                            })
                                        })
                                        .collect::<Result<Vec<Point>, ParseError>>()?,
                                })
                            })
                            .collect::<Result<Vec<Line>, ParseError>>()?;
                        Ok(Layer { lines })
                    })
                    .collect::<Result<Vec<Layer>, ParseError>>()?;

                Ok(Page { layers })
            })
            .collect::<Result<Vec<Page>, ParseError>>()?;

        return Ok(RemarkableFile { version, pages });
    }
}
