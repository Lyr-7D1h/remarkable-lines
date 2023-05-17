use std::{collections::HashMap, io::Read};

use crate::{
    bitreader::Readable,
    v6::{
        block::TypeParse,
        crdt::{CrdtId, CrdtSequenceItem},
        tagged_bit_reader::TaggedBitreader,
    },
    ParseError,
};

use super::{group::Group, text::Text, BlockInfo, BlockParse};

#[derive(Debug)]
pub struct MigrationInfoBlock {
    migration_id: CrdtId,
    is_device: bool,
}
impl BlockParse for MigrationInfoBlock {
    fn parse(
        info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let migration_id = reader.read_id(1)?;

        let is_device = reader.read_u8(2)? > 0;

        if info.has_bytes_remaining(&mut reader.bit_reader) {
            _ = reader.bit_reader.read_u8();
        }
        Ok(Self {
            migration_id,
            is_device,
        })
    }
}

#[derive(Debug)]
pub struct AuthorsIdsBlock {
    authors: HashMap<u16, String>,
}
impl BlockParse for AuthorsIdsBlock {
    fn parse(
        _info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let amount_subblocks = reader.bit_reader.read_varuint()?;
        let mut authors = HashMap::new();

        for _ in 0..amount_subblocks {
            let block = reader.read_subblock(0)?;

            let uuid = reader.bit_reader.read_uuid()?;

            let author_id = reader.bit_reader.read_u16()?;
            authors.insert(author_id, uuid);
            block.validate_size(reader);
        }

        Ok(Self { authors })
    }
}

#[derive(Debug)]
pub struct PageInfoBlock {
    loads_count: u32,
    merges_count: u32,
    text_chars_count: u32,
    text_lines_count: u32,
}
impl BlockParse for PageInfoBlock {
    fn parse(
        info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let loads_count = reader.read_u32(1)?;
        let merges_count = reader.read_u32(2)?;
        let text_chars_count = reader.read_u32(3)?;
        let text_lines_count = reader.read_u32(4)?;

        if info.has_bytes_remaining(&mut reader.bit_reader) {
            reader.read_u32(5)?;
        }

        Ok(Self {
            loads_count,
            merges_count,
            text_chars_count,
            text_lines_count,
        })
    }
}

#[derive(Debug)]
pub struct TreeNodeBlock {
    group: Group,
}
impl BlockParse for TreeNodeBlock {
    fn parse(
        info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let mut group = Group::default();
        group.node_id = reader.read_id(1)?;
        group.label = reader.read_lww_string(2)?;
        group.visible = reader.read_lww_bool(3)?;

        if info.has_bytes_remaining(&reader.bit_reader) {
            group.anchor_id = Some(reader.read_lww_id(7)?);
            group.anchor_type = Some(reader.read_lww_u8(8)?);
            group.anchor_threshold = Some(reader.read_lww_f32(9)?);
            group.anchor_origin_x = Some(reader.read_lww_f32(10)?);
        }

        Ok(Self { group })
    }
}

#[derive(Debug)]
pub struct SceneTreeBlock {
    tree_id: CrdtId,
    node_id: CrdtId,
    is_update: bool,
    parent_id: CrdtId,
}
impl BlockParse for SceneTreeBlock {
    fn parse(
        _info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let tree_id = reader.read_id(1)?;
        let node_id = reader.read_id(2)?;
        let is_update = reader.read_bool(3)?;

        let subblock = reader.read_subblock(4)?;
        let parent_id = reader.read_id(1)?;
        subblock.validate_size(reader)?;

        Ok(Self {
            tree_id,
            node_id,
            is_update,
            parent_id,
        })
    }
}
#[derive(Debug)]
pub struct SceneGlyphItem {}
impl BlockParse for SceneGlyphItem {
    fn parse(
        info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct RootTextBlock {
    block_id: CrdtId,
    text: Text,
}
impl BlockParse for RootTextBlock {
    fn parse(
        info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let block_id = reader.read_id(1)?;

        Ok(RootTextBlock {
            block_id,
            text: Text::parse(reader)?,
        })
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
enum SceneItemType {
    SceneGlyphItemBlock = 1,
    SceneGroupItemBlock = 2,
    SceneLineItemBlock = 3,
    SceneTextItemBlock = 5,
}
impl SceneItemType {
    pub fn validate(self, scene_item_type: SceneItemType) -> Result<(), ParseError> {
        if self != scene_item_type {
            return Err(ParseError::invalid(format!(
                "Invalid scene item type given '{:?}' expected '{:?}'",
                self, scene_item_type
            )));
        }

        Ok(())
    }
}
impl TryFrom<u8> for SceneItemType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SceneItemType::SceneGlyphItemBlock),
            2 => Ok(SceneItemType::SceneGroupItemBlock),
            3 => Ok(SceneItemType::SceneLineItemBlock),
            5 => Ok(SceneItemType::SceneTextItemBlock),
            _ => Err(ParseError::invalid(format!(
                "Invalid scene item type with value '{value}'"
            ))),
        }
    }
}
#[derive(Debug)]
pub struct SceneItem<N> {
    parent_id: CrdtId,
    item: CrdtSequenceItem<Option<N>>,
}
impl<N> SceneItem<N> {
    fn parse<R: Readable>(
        reader: &mut TaggedBitreader<R>,
        scene_item_type: SceneItemType,
        get_value: fn(&mut TaggedBitreader<R>) -> Result<N, ParseError>,
    ) -> Result<Self, ParseError> {
        let parent_id = reader.read_id(1)?;
        let item_id = reader.read_id(2)?;
        let left_id = reader.read_id(3)?;
        let right_id = reader.read_id(4)?;
        let deleted_length = reader.read_u32(5)?;

        let value = if reader.has_subblock(6)? {
            let subblock = reader.read_subblock(6)?;
            SceneItemType::try_from(reader.bit_reader.read_u8()?)?.validate(scene_item_type)?;
            let value = get_value(reader)?;
            subblock.validate_size(reader)?;
            Some(value)
        } else {
            None
        };

        Ok(SceneItem {
            parent_id,
            item: CrdtSequenceItem {
                left_id,
                item_id,
                right_id,
                deleted_length,
                value,
            },
        })
    }
}

#[derive(Debug)]
pub struct SceneGroupItemBlock {
    // XXX don't know what this means
    scene_item_block: SceneItem<CrdtId>,
}
impl BlockParse for SceneGroupItemBlock {
    fn parse(
        info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        Ok(Self {
            scene_item_block: SceneItem::parse(reader, SceneItemType::SceneGroupItemBlock, |r| {
                // XXX don't know what this means
                r.read_id(2)
            })?,
        })
    }
}
