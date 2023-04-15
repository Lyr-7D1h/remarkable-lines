use std::collections::HashMap;

use crate::ParseError;

use super::{
    crdtid::CrdtId,
    tag::{Tag, TagType},
    TypeParse,
};

#[derive(Debug)]
pub struct LwwValue<T> {
    timestamp: CrdtId,
    value: T,
}

#[derive(Debug)]
/// Text paragraph style.
pub enum ParagraphStyle {
    BASIC,
    PLAIN,
    HEADING,
    BOLD,
    BULLET,
    BULLET2,
}

impl TryFrom<u8> for ParagraphStyle {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ParagraphStyle::BASIC),
            0x01 => Ok(ParagraphStyle::PLAIN),
            0x02 => Ok(ParagraphStyle::HEADING),
            0x03 => Ok(ParagraphStyle::BOLD),
            0x04 => Ok(ParagraphStyle::BULLET),
            0x05 => Ok(ParagraphStyle::BULLET2),
            _ => Err(ParseError::invalid(format!(
                "Invalid paragraph style with value '{value}'"
            ))),
        }
    }
}

// pub struct TextItem<T> {
//     item_id: CrdtId,
//     left_id: CrdtId,
//     right_id: CrdtId,
//     deleted_length: u32,
//     value: T,
// }

#[derive(Debug)]
pub enum TextItem {
    FormatCode(u32),
    Text(String),
}

#[derive(Debug)]
/// Block of text
pub struct Text {
    items: Vec<TextItem>,
    styles: HashMap<CrdtId, LwwValue<ParagraphStyle>>,
    x: f64,
    y: f64,
    width: f32,
}
impl TypeParse for Text {
    fn parse<N: std::io::Read>(
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, crate::ParseError> {
        Tag::parse(reader)?.validate(TagType::Length4, 2)?;
        let _length = reader.read_u32()?;
        Tag::parse(reader)?.validate(TagType::Length4, 3)?;
        let _length = reader.read_u32()?;

        Tag::parse(reader)?.validate(TagType::Length4, 2)?;
        let mut amount_subblocks = reader.read_varuint()?;
        let items = (0..amount_subblocks)
            .into_iter()
            .map(|_| {
                Tag::parse(reader)?.validate(TagType::Length4, 1)?;
                let _length = reader.read_u32()?;
                Tag::parse(reader)?.validate(TagType::ID, 2)?;
                let item_id = CrdtId::parse(reader)?;
                Tag::parse(reader)?.validate(TagType::ID, 3)?;
                let left_id = CrdtId::parse(reader)?;
                Tag::parse(reader)?.validate(TagType::ID, 4)?;
                let right_id = CrdtId::parse(reader)?;
                let deleted_length = reader.read_u32();

                Tag::parse(reader)?.validate(TagType::Length4, 5)?;
                let _length = reader.read_u32()?;
                let string_length = reader.read_varuint()?;
                // XXX might have a different meaning
                let is_ascii = reader.read_bool()?;
                let string = reader.read_string(string_length as usize)?;

                // FIXME peek to check for tag
                // if tag_exists {
                //     Tag::parse(reader)?.validate(TagType::Byte4, 2)?;
                //     let fmt_code = reader.read_u32()?;
                //     Ok(TextItem::FormatCode(fmt_code))
                // } else {
                Ok(TextItem::Text(string))
                // }
            })
            .collect::<Result<Vec<TextItem>, ParseError>>()?;

        Tag::parse(reader)?.validate(TagType::Length4, 2)?;
        let mut amount_subblocks = reader.read_varuint()?;
        let styles = (0..amount_subblocks)
            .into_iter()
            .map(|_| {
                let id = CrdtId::parse(reader)?;
                Tag::parse(reader)?.validate(TagType::ID, 2)?;
                let timestamp = CrdtId::parse(reader)?;

                Tag::parse(reader)?.validate(TagType::Length4, 2)?;
                let _length = reader.read_varuint()?;
                let value = ParagraphStyle::try_from(reader.read_u8()?)?;
                Ok((id, LwwValue { timestamp, value }))
            })
            .collect::<Result<HashMap<CrdtId, LwwValue<ParagraphStyle>>, ParseError>>()?;

        Tag::parse(reader)?.validate(TagType::Length4, 2)?;
        let _length = reader.read_varuint()?;
        let x = reader.read_f64()?;
        let y = reader.read_f64()?;
        Tag::parse(reader)?.validate(TagType::Length4, 2)?;
        let width = reader.read_f32()?;
        Ok(Text {
            items,
            styles,
            x,
            y,
            width,
        })
    }
}
