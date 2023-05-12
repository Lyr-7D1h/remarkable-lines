use crate::{bitreader::Readable, Bitreader, ParseError};

use super::{crdt::CrdtId, TypeParse};

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
struct Tag {
    index: u32,
    tag_type: TagType,
}

impl Tag {
    fn has_tag(
        reader: &mut Bitreader<impl Readable>,
        tag_type: TagType,
        tag_index: u32,
    ) -> Result<bool, crate::ParseError> {
        let pos = reader.position();
        let has_tag = Tag::parse(reader)?.validate(tag_type, tag_index).is_ok();
        reader.set_position(pos);
        return Ok(has_tag);
    }

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
    fn parse(reader: &mut crate::Bitreader<impl Readable>) -> Result<Self, ParseError> {
        let x = reader.read_varuint()?;
        Ok(Tag {
            index: x >> 4,
            tag_type: TagType::try_from(x & 0xF)?,
        })
    }
}

struct TaggedBitReader<N: Readable> {
    pub bit_reader: Bitreader<N>,
}

impl<N: Readable> TaggedBitReader<N> {
    pub fn new(bit_reader: Bitreader<N>) -> TaggedBitReader<N> {
        TaggedBitReader { bit_reader }
    }

    pub fn read_id(&mut self, index: u32) -> Result<CrdtId, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::ID, index);
        return CrdtId::parse(&mut self.bit_reader);
    }

    pub fn read_bool(&mut self, index: u32) -> Result<bool, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::ID, index);
        return self.bit_reader.read_bool();
    }

    pub fn read_u8(&mut self, index: u32) -> Result<bool, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::Byte1, index);
        return self.bit_reader.read_u8();
    }

    pub fn read_u32(&mut self, index: u32) -> Result<bool, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::Byte4, index);
        return self.bit_reader.read_u32();
    }

    pub fn read_f32(&mut self, index: u32) -> Result<bool, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::Byte4, index);
        return self.bit_reader.read_f32();
    }

    pub fn read_f64(&mut self, index: u32) -> Result<bool, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::Byte8, index);
        return self.bit_reader.read_f64();
    }

    pub fn read_bool(&mut self, index: u32) -> Result<bool, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::ID, index);
        return self.bit_reader.read_bool();
    }

    pub fn read_bool(&mut self, index: u32) -> Result<bool, ParseError> {
        Tag::parse(&mut self.bit_reader)?.validate(TagType::ID, index);
        return self.bit_reader.read_bool();
    }

    pub fn read_subblock(&mut self) -> Result<Self, crate::ParseError>
    where
        Self: Sized,
    {
        let tag = Tag::parse(reader)?;
        let size = reader.read_u32()?;
        let position = reader.position();

        Ok(Self {
            tag,
            size,
            position,
        })
    }

    pub fn read_lww_u8(
        reader: &mut Bitreader<impl Readable>,
        index: u32,
    ) -> Result<LwwValue<u8>, ParseError> {
        let subblock1 = SubBlock::parse(reader)?.validate_tag(index)?;

        Tag::parse(reader)?.validate(super::tag::TagType::ID, 1);
        let timestamp = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(super::tag::TagType::Byte1, 2);
        let value = reader.read_u8()?;

        subblock1.validate_size(reader);

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_string(
        reader: &mut Bitreader<impl Readable>,
        index: u32,
    ) -> Result<LwwValue<String>, ParseError> {
        let subblock1 = SubBlock::parse(reader)?.validate_tag(index)?;

        Tag::parse(reader)?.validate(super::tag::TagType::ID, 1);
        let timestamp = CrdtId::parse(reader)?;

        let subblock2 = SubBlock::parse(reader)?.validate_tag(2)?;
        let length = reader.read_varuint()?;
        let is_ascii = reader.read_bool()?;
        let value = reader.read_string(length.try_into()?)?;
        subblock2.validate_size(reader)?;

        subblock1.validate_size(reader);

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_bool(
        reader: &mut Bitreader<impl Readable>,
        index: u32,
    ) -> Result<LwwValue<bool>, ParseError> {
        let subblock1 = SubBlock::parse(reader)?.validate_tag(index)?;

        Tag::parse(reader)?.validate(super::tag::TagType::ID, 1);
        let timestamp = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(super::tag::TagType::Byte1, 2);
        let value = reader.read_bool()?;

        subblock1.validate_size(reader);

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_id(
        reader: &mut Bitreader<impl Readable>,
        index: u32,
    ) -> Result<LwwValue<CrdtId>, ParseError> {
        let subblock1 = SubBlock::parse(reader)?.validate_tag(index)?;

        Tag::parse(reader)?.validate(super::tag::TagType::ID, 1);
        let timestamp = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(super::tag::TagType::ID, 2);
        let value = CrdtId::parse(reader)?;

        subblock1.validate_size(reader);

        Ok(LwwValue { timestamp, value })
    }

    pub fn read_lww_float(
        reader: &mut Bitreader<impl Readable>,
        index: u32,
    ) -> Result<LwwValue<f32>, ParseError> {
        let subblock1 = SubBlock::parse(reader)?.validate_tag(index)?;

        Tag::parse(reader)?.validate(super::tag::TagType::ID, 1);
        let timestamp = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(super::tag::TagType::Byte4, 2);
        let value = reader.read_f32()?;

        subblock1.validate_size(reader);

        Ok(LwwValue { timestamp, value })
    }
}
