use std::io::Read;

use line::Line;
use regex::Regex;

pub mod color;
pub mod line;
pub mod parse_error;
pub mod point;
pub mod tool;

pub use crate::parse_error::ParseErrorKind;
pub use color::Color;
pub use parse_error::ParseError;
pub use point::Point;
pub use tool::Tool;

pub struct RemarkableFile {
    version: u8,
    layers: Vec<Vec<Line>>,
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
        let file_description: String = file_description.into_iter().map(|i| i as char).collect();

        let range = Regex::new("version=[0-9]+")
            .unwrap()
            .find(&file_description)
            .ok_or(ParseError::invalid("Could not find version"))?;

        let version = file_description[range.start()..range.end()]
            .split("=")
            .last()
            .ok_or(ParseError::invalid("Could not find version"))?
            .parse()?;

        if version != 5 {
            return Err(ParseError::new(
                &format!("version '{version}' is not supported"),
                ParseErrorKind::Unsupported,
            ));
        }

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
                Ok(lines)
            })
            .collect::<Result<Vec<Vec<Line>>, ParseError>>()?;

        return Ok(RemarkableFile { version, layers });
    }

    pub fn version(&self) -> u8 {
        self.version
    }
}
