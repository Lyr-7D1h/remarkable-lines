use crate::ParseError;

/// Data representation of an exported color in a reMarkable document line
#[derive(Debug, Clone)]
pub enum PenColor {
    Black,
    Grey,
    White,
    Yellow,
    Green,
    Pink,
    Blue,
    Red,
    GreyOverlap,
}

impl TryFrom<u32> for PenColor {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PenColor::Black),
            0x01 => Ok(PenColor::Grey),
            0x02 => Ok(PenColor::White),
            0x03 => Ok(PenColor::Yellow),
            0x04 => Ok(PenColor::Green),
            0x05 => Ok(PenColor::Pink),
            0x06 => Ok(PenColor::Blue),
            0x07 => Ok(PenColor::Red),
            0x08 => Ok(PenColor::GreyOverlap),
            _ => Err(ParseError::invalid(format!(
                "Invalid color with value '{value}'"
            ))),
        }
    }
}
