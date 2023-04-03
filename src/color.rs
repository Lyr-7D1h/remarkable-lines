use crate::ParseError;

/// Data representation of an exported color in a reMarkable document line
#[derive(Debug)]
pub enum Color {
    Black,
    Grey,
    White,
    Blue,
    Red,
}

impl TryFrom<u32> for Color {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Color::Black),
            0x01 => Ok(Color::Grey),
            0x02 => Ok(Color::White),
            0x06 => Ok(Color::Red),
            0x07 => Ok(Color::Blue),
            _ => Err(ParseError::invalid(format!(
                "Invalid color with value '{value}'"
            ))),
        }
    }
}
