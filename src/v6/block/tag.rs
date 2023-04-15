use crate::ParseError;

use super::BlockInfo;
use super::BlockParse;
use super::TypeParse;

#[derive(Debug, PartialEq)]
pub enum TagType {
    ID,
    Length4,
    Byte8,
    Byte4,
    Byte1,
}

impl TryFrom<u32> for TagType {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(TagType::Byte1),
            0x4 => Ok(TagType::Byte4),
            0x8 => Ok(TagType::Byte8),
            0xC => Ok(TagType::Length4),
            0x0F => Ok(TagType::ID),
            _ => Err(ParseError::invalid(format!(
                "Invalid tool with value '{value}'"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct Tag {
    pub index: u32,
    pub tag_type: TagType,
}

impl Tag {
    /// Helper function to easily generate errors and to validate
    pub fn validate(&self, tag_type: TagType, index: u32) -> Result<(), ParseError> {
        if self.tag_type != tag_type {
            return Err(ParseError::invalid(format!(
                "Invalid tag type given '{:?}' expected '{:?}'",
                self.tag_type, tag_type
            )));
        }

        if self.index != index {
            return Err(ParseError::invalid(format!(
                "Invalid tag index given '{:?}' expected '{:?}'",
                self.index, index
            )));
        }

        Ok(())
    }
}

impl TypeParse for Tag {
    fn parse<N: std::io::Read>(reader: &mut crate::Bitreader<N>) -> Result<Self, ParseError> {
        let x = reader.read_varuint()?;
        Ok(Tag {
            index: x >> 4,
            tag_type: TagType::try_from(x & 0xF)?,
        })
    }
}
