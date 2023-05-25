use std::collections::HashMap;

use crate::{
    bitreader::Readable,
    v6::{
        crdt::CrdtId,
        lwwvalue::LwwValue,
        tagged_bit_reader::{TagType, TaggedBitreader},
        TypeParse,
    },
    ParseError,
};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum TextItem {
    FormatCode(u32),
    Text(String),
}

#[derive(Debug, Clone)]
/// Block of text
pub struct Text {
    items: Vec<TextItem>,
    styles: HashMap<CrdtId, LwwValue<ParagraphStyle>>,
    x: f64,
    y: f64,
    width: f32,
}
impl TypeParse for Text {
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, crate::ParseError> {
        // subblocks
        let subblock1 = reader.read_subblock(2)?;
        let subblock2 = reader.read_subblock(1)?;
        let subblock3 = reader.read_subblock(1)?;

        // Text items
        let amount_items = reader.bit_reader.read_varuint()?;
        let items = (0..amount_items)
            .into_iter()
            .map(|_| {
                let subblock = reader.read_subblock(0)?;
                let item_id = reader.read_id(2)?;
                let left_id = reader.read_id(3)?;
                let right_id = reader.read_id(4)?;
                let deleted_length = reader.read_u32(5)?;

                if reader.has_subblock(6)? {
                    let subblock = reader.read_subblock(6)?;

                    let string_length = reader.bit_reader.read_varuint()?;
                    // XXX might have a different meaning
                    let is_ascii = reader.bit_reader.read_bool()?;
                    let string = reader.bit_reader.read_string(string_length as usize)?;

                    // if tag exists use format
                    if reader.has_tag(2, TagType::Byte4)? {
                        let fmt_code = reader.read_u32(2)?;
                        return Ok(TextItem::FormatCode(fmt_code));
                    }
                    subblock.validate_size(reader)?;
                    return Ok(TextItem::Text(string));
                }
                subblock.validate_size(reader)?;

                return Ok(TextItem::Text(String::new()));
            })
            .collect::<Result<Vec<TextItem>, ParseError>>()?;

        subblock2.validate_size(reader)?;
        subblock3.validate_size(reader)?;

        let subblock4 = reader.read_subblock(2)?;
        let subblock5 = reader.read_subblock(1)?;

        // Formatting
        let amount_styles = reader.bit_reader.read_varuint()?;
        let styles = (0..amount_styles)
            .into_iter()
            .map(|_| {
                let id = CrdtId::parse(reader)?;
                let timestamp = reader.read_id(1)?;

                let subblock6 = reader.read_subblock(2)?;
                // XXX not sure what this is format?
                let _c = reader.bit_reader.read_u8()?;
                let style = ParagraphStyle::try_from(reader.bit_reader.read_u8()?)?;
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

        println!("{:x}", reader.bit_reader.position());

        // Last section
        // "pos_x" and "pos_y" from ddvk? Gives negative number -- possibly could
        // be bounding box?
        let subblock7 = reader.read_subblock(3)?;
        let x = reader.bit_reader.read_f64()?;
        let y = reader.bit_reader.read_f64()?;
        subblock7.validate_size(reader)?;

        let width = reader.read_f32(4)?;

        Ok(Text {
            items,
            styles,
            x,
            y,
            width,
        })
    }
}
