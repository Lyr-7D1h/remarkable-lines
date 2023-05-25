use crate::{bitreader::Readable, Bitreader, ParseError};

use super::{crdt::CrdtId, lwwvalue::LwwValue, TypeParse};

pub struct SubBlock {
    pub tag: Tag,
    pub size: u32,
    pub position: u64,
}

impl SubBlock {
    pub fn validate_size(
        &self,
        reader: &TaggedBitreader<impl Readable>,
    ) -> Result<(), crate::ParseError> {
        let expected_offset = self.position + self.size as u64;
        let end_offset = reader.bit_reader.position();
        if expected_offset != end_offset {
            return Err(ParseError::invalid(format!(
                "Did not read expected size of subblock. got {end_offset:x} expected {expected_offset:x}" 
            )));
        }
        Ok(())
    }
}

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
                "Invalid tag for value '{value}'"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct Tag {
    index: u32,
    tag_type: TagType,
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
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, ParseError> {
        let x = reader.bit_reader.read_varuint()?;
        Ok(Tag {
            index: x >> 4,
            tag_type: TagType::try_from(x & 0xF)?,
        })
    }
}

pub struct TaggedBitreader<'n, N: Readable> {
    pub bit_reader: &'n mut Bitreader<N>,
}

impl<'n, N: Readable> TaggedBitreader<'n, N> {
    pub fn new(bit_reader: &'n mut Bitreader<N>) -> TaggedBitreader<'n, N> {
        TaggedBitreader { bit_reader }
    }

    pub fn read_id(&mut self, index: u32) -> Result<CrdtId, ParseError> {
        self.read_tag(index, TagType::ID)?;
        return CrdtId::parse(self);
    }

    pub fn read_bool(&mut self, index: u32) -> Result<bool, ParseError> {
        self.read_tag(index, TagType::Byte1)?;
        return self.bit_reader.read_bool();
    }

    pub fn read_u8(&mut self, index: u32) -> Result<u8, ParseError> {
        self.read_tag(index, TagType::Byte1)?;
        return self.bit_reader.read_u8();
    }

    pub fn read_u32(&mut self, index: u32) -> Result<u32, ParseError> {
        self.read_tag(index, TagType::Byte4)?;
        return self.bit_reader.read_u32();
    }

    pub fn read_f32(&mut self, index: u32) -> Result<f32, ParseError> {
        self.read_tag(index, TagType::Byte4)?;
        return self.bit_reader.read_f32();
    }

    pub fn read_f64(&mut self, index: u32) -> Result<f64, ParseError> {
        self.read_tag(index, TagType::Byte8)?;
        return self.bit_reader.read_f64();
    }

    pub fn read_string(&mut self, index: u32) -> Result<String, ParseError> {
        let subblock = self.read_subblock(index)?;
        let string_length = self.bit_reader.read_varuint()?;
        let is_ascii = self.bit_reader.read_bool()?;
        let string = self.bit_reader.read_string(string_length as usize)?;
        subblock.validate_size(self);
        return Ok(string);
    }

    pub fn read_tag(&mut self, index: u32, tag_type: TagType) -> Result<Tag, ParseError> {
        let x = self.bit_reader.read_varuint()?;

        let tag = Tag {
            index: x >> 4,
            tag_type: TagType::try_from(x & 0xF)?,
        };
        tag.validate(tag_type, index)?;

        Ok(tag)
    }

    pub fn has_tag(&mut self, index: u32, tag_type: TagType) -> Result<bool, crate::ParseError> {
        let pos = self.bit_reader.position();
        let has_tag = self.read_tag(index, tag_type).is_ok();
        self.bit_reader.set_position(pos);
        return Ok(has_tag);
    }

    pub fn read_subblock(&mut self, index: u32) -> Result<SubBlock, crate::ParseError>
    where
        Self: Sized,
    {
        let tag = self.read_tag(index, TagType::Length4)?;
        let size = self.bit_reader.read_u32()?;
        let position = self.bit_reader.position();

        Ok(SubBlock {
            tag,
            size,
            position,
        })
    }

    pub fn has_subblock(&mut self, index: u32) -> Result<bool, ParseError> {
        return self.has_tag(index, TagType::Length4);
    }

    pub fn read_lww_u8(&mut self, index: u32) -> Result<LwwValue<u8>, ParseError> {
        let subblock = self.read_subblock(index)?;

        let timestamp = self.read_id(1)?;
        let value = self.read_u8(2)?;

        subblock.validate_size(self)?;

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_string(&mut self, index: u32) -> Result<LwwValue<String>, ParseError> {
        let subblock = self.read_subblock(index)?;

        let timestamp = self.read_id(1)?;

        let subblock2 = self.read_subblock(2)?;
        let length = self.bit_reader.read_varuint()?;
        let _is_ascii = self.bit_reader.read_bool()?;
        let value = self.bit_reader.read_string(length.try_into()?)?;
        subblock2.validate_size(self)?;
        subblock.validate_size(self)?;

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_bool(&mut self, index: u32) -> Result<LwwValue<bool>, ParseError> {
        let subblock = self.read_subblock(index)?;

        let timestamp = self.read_id(1)?;
        let value = self.read_bool(2)?;

        subblock.validate_size(self)?;

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_id(&mut self, index: u32) -> Result<LwwValue<CrdtId>, ParseError> {
        let subblock = self.read_subblock(index)?;

        let timestamp = self.read_id(1)?;
        let value = self.read_id(2)?;

        subblock.validate_size(self)?;

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_f32(&mut self, index: u32) -> Result<LwwValue<f32>, ParseError> {
        let subblock = self.read_subblock(index)?;

        let timestamp = self.read_id(1)?;
        let value = self.read_f32(2)?;

        subblock.validate_size(self)?;

        Ok(LwwValue { timestamp, value })
    }
}
