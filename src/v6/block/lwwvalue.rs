use crate::{bitreader::Readable, Bitreader, ParseError};

use super::{crdt::CrdtId, subblock::SubBlock, tag::Tag, TypeParse};

#[derive(Debug)]
pub struct LwwValue<T> {
    pub timestamp: CrdtId,
    pub value: T,
}

impl<T> LwwValue<T> {
    pub fn read_u8(
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
