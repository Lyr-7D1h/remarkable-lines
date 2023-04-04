use std::io::Read;

use layer::Layer;
use line::Line;
use page::Page;

pub mod color;
pub mod layer;
pub mod line;
pub mod page;
pub mod parse_error;
pub mod point;
pub mod tool;

pub use crate::parse_error::ParseErrorKind;
pub use color::Color;
pub use parse_error::ParseError;
pub use point::Point;
pub use tool::Tool;

#[derive(Debug)]
pub struct RemarkableFile {
    version: u8,
    pages: Vec<Page>,
}

fn read_f32(input: &mut impl Read) -> Result<f32, ParseError> {
    let mut buffer = [0; 4];
    input.read_exact(&mut buffer)?;
    return Ok(f32::from_le_bytes(buffer));
}

fn read_u32(input: &mut impl Read) -> Result<u32, ParseError> {
    let mut buffer = [0; 4];
    input.read_exact(&mut buffer)?;
    return Ok(u32::from_le_bytes(buffer));
}

impl RemarkableFile {
    /// Parse a remarkable file in little endian order
    pub fn read(mut input: impl Read) -> Result<RemarkableFile, ParseError> {
        let mut file_description = [0; 43];
        input.read_exact(&mut file_description)?;
        let version_description: String = file_description.into_iter().map(|i| i as char).collect();

        let version = match version_description.trim_end() {
            "reMarkable lines with selections and layers" => {
                // early version of the format that is not supported
                return Err(ParseError::unsupported("Unsupported outdated version"));
            }
            "reMarkable .lines file, version=3" => 3,
            "reMarkable .lines file, version=5" => 5,
            _ => {
                return Err(ParseError::unsupported(format!(
                    "Unknown version from: {version_description}"
                )))
            }
        };

        if version != 5 {
            return Err(ParseError::new(
                &format!("version '{version}' is not supported"),
                ParseErrorKind::Unsupported,
            ));
        }

        let amount_pages = if version >= 3 {
            1
        } else {
            read_u32(&mut input)?
        };

        let pages = (0..amount_pages)
            .map(|_| {
                let amount_layers = read_u32(&mut input)?;
                let layers = (0..amount_layers)
                    .map(|_| {
                        let amount_lines = read_u32(&mut input)?;
                        let lines = (0..amount_lines)
                            .map(|_| {
                                let tool = Tool::try_from(read_u32(&mut input)?)?;
                                let color = Color::try_from(read_u32(&mut input)?)?;
                                let unknown_line_attribute = read_u32(&mut input)?;
                                let padding = read_u32(&mut input)?;
                                let brush_size = read_f32(&mut input)?;
                                let unknown_line_attribute_2 = if version >= 5 {
                                    read_u32(&mut input)?
                                } else {
                                    0
                                };
                                let amount_points = read_u32(&mut input)?;
                                println!("{amount_points}");
                                let points = (0..amount_points)
                                    .map(|_| {
                                        // TODO try moving in struct
                                        let x = read_f32(&mut input)?;
                                        let y = read_f32(&mut input)?;
                                        let speed = read_f32(&mut input)?;
                                        let direction = read_f32(&mut input)?;
                                        let width = read_f32(&mut input)?;
                                        let pressure = read_f32(&mut input)?;

                                        Ok(Point {
                                            x,
                                            y,
                                            speed,
                                            direction,
                                            width,
                                            pressure,
                                        })
                                    })
                                    .collect::<Result<Vec<Point>, ParseError>>()?;

                                Ok(Line {
                                    unknown_line_attribute,
                                    unknown_line_attribute_2,
                                    tool,
                                    color,
                                    padding,
                                    brush_size,
                                    points,
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

    pub fn version(&self) -> u8 {
        self.version
    }
}