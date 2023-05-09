use std::collections::HashMap;

use crate::{bitreader::Readable, ParseError};

use super::{
    crdt::CrdtId,
    lwwvalue::LwwValue,
    subblock::SubBlock,
    tag::{Tag, TagType},
    TypeParse,
};

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
    fn parse(reader: &mut crate::Bitreader<impl Readable>) -> Result<Self, crate::ParseError> {
        // subblocks
        let subblock1 = SubBlock::parse(reader)?.validate_tag(2)?;
        let subblock2 = SubBlock::parse(reader)?.validate_tag(1)?;
        let subblock3 = SubBlock::parse(reader)?.validate_tag(1)?;

        // Text items
        let amount_items = reader.read_varuint()?;
        let items = (0..amount_items)
            .into_iter()
            .map(|_| {
                Tag::parse(reader)?.validate(TagType::Length4, 0)?;
                let _length = reader.read_u32()?;
                Tag::parse(reader)?.validate(TagType::ID, 2)?;
                let item_id = CrdtId::parse(reader)?;
                Tag::parse(reader)?.validate(TagType::ID, 3)?;
                let left_id = CrdtId::parse(reader)?;
                Tag::parse(reader)?.validate(TagType::ID, 4)?;
                let right_id = CrdtId::parse(reader)?;
                Tag::parse(reader)?.validate(TagType::Byte4, 5)?;
                let deleted_length = reader.read_u32();

                let pos = reader.position();
                if let Ok(_) = Tag::parse(reader)?.validate(TagType::Length4, 6) {
                    let _length = reader.read_u32()?;

                    let string_length = reader.read_varuint()?;
                    // XXX might have a different meaning
                    let is_ascii = reader.read_bool()?;
                    let string = reader.read_string(string_length as usize)?;

                    // if tag exists use format
                    let pos = reader.position();
                    if let Ok(_) = Tag::parse(reader)?.validate(TagType::Byte4, 2) {
                        let fmt_code = reader.read_u32()?;
                        return Ok(TextItem::FormatCode(fmt_code));
                    }
                    reader.set_position(pos); // reset

                    return Ok(TextItem::Text(string));
                }

                reader.set_position(pos);
                return Ok(TextItem::Text(String::new()));
            })
            .collect::<Result<Vec<TextItem>, ParseError>>()?;

        subblock2.validate_size(reader)?;
        subblock3.validate_size(reader)?;

        let subblock4 = SubBlock::parse(reader)?.validate_tag(2)?;
        let subblock5 = SubBlock::parse(reader)?.validate_tag(1)?;

        // Formatting
        let amount_styles = reader.read_varuint()?;
        let styles = (0..amount_styles)
            .into_iter()
            .map(|_| {
                let id = CrdtId::parse(reader)?;
                Tag::parse(reader)?.validate(TagType::ID, 1)?;
                let timestamp = CrdtId::parse(reader)?;

                let subblock6 = SubBlock::parse(reader)?.validate_tag(2)?;
                // XXX not sure what this is format?
                let _c = reader.read_u8()?;
                let style = ParagraphStyle::try_from(reader.read_u8()?)?;
                subblock6.validate_size(reader)?;
                Ok((
                    id,
                    LwwValue {
                        timestamp,
                        value: style,
                    },
                ))
            })
            .collect::<Result<HashMap<CrdtId, LwwValue<ParagraphStyle>>, ParseError>>()?;

        subblock4.validate_size(reader)?;
        subblock5.validate_size(reader)?;

        subblock1.validate_size(reader)?;

        // Last section
        // "pos_x" and "pos_y" from ddvk? Gives negative number -- possibly could
        // be bounding box?
        let subblock7 = SubBlock::parse(reader)?.validate_tag(3)?;
        let x = reader.read_f64()?;
        let y = reader.read_f64()?;
        subblock7.validate_size(reader)?;

        Tag::parse(reader)?.validate(TagType::Byte4, 4)?;
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
