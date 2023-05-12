use std::collections::HashMap;

use crate::{
    bitreader::Readable,
    v6::block::{
        tag::{Tag, TagType},
        TypeParse,
    },
    ParseError,
};

use super::{
    crdt::CrdtId, group::Group, lwwvalue::LwwValue, subblock::SubBlock, text::Text, BlockInfo,
    BlockParse,
};

#[derive(Debug)]
pub struct MigrationInfoBlock {
    migration_id: CrdtId,
    is_device: bool,
}
impl BlockParse for MigrationInfoBlock {
    fn parse(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        Tag::parse(reader)?.validate(TagType::ID, 1)?;
        let migration_id = CrdtId::parse(reader)?;

        Tag::parse(reader)?.validate(TagType::Byte1, 2)?;
        let is_device = reader.read_u8()? > 0;

        if info.has_bytes_remaining(reader) {
            _ = reader.read_u8();
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
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let amount_subblocks = reader.read_varuint()?;
        let mut authors = HashMap::new();

        for _ in 0..amount_subblocks {
            Tag::parse(reader)?.validate(TagType::Length4, 0)?;

            let _subblock_length = reader.read_u32()?;

            let uuid = reader.read_uuid()?;

            let author_id = reader.read_u16()?;
            authors.insert(author_id, uuid);
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
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        Tag::parse(reader)?.validate(TagType::Byte4, 1)?;
        let loads_count = reader.read_u32()?;

        Tag::parse(reader)?.validate(TagType::Byte4, 2)?;
        let merges_count = reader.read_u32()?;

        Tag::parse(reader)?.validate(TagType::Byte4, 3)?;
        let text_chars_count = reader.read_u32()?;

        Tag::parse(reader)?.validate(TagType::Byte4, 4)?;
        let text_lines_count = reader.read_u32()?;

        if info.has_bytes_remaining(reader) {
            let _unknown = Tag::parse(reader)?.validate(TagType::Byte4, 5)?;
            reader.read_u32()?;
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
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        let mut group = Group::default();
        Tag::parse(reader)?.validate(TagType::ID, 1)?;
        group.node_id = CrdtId::parse(reader)?;
        group.label = LwwValue::<String>::read_lww_string(reader, 2)?;
        group.visible = LwwValue::<bool>::read_lww_bool(reader, 3)?;

        if info.has_bytes_remaining(reader) {
            group.anchor_id = Some(LwwValue::<CrdtId>::read_lww_id(reader, 7)?);
            group.anchor_type = Some(LwwValue::<u8>::read_u8(reader, 8)?);
            group.anchor_threshold = Some(LwwValue::<f32>::read_lww_float(reader, 9)?);
            group.anchor_origin_x = Some(LwwValue::<f32>::read_lww_float(reader, 10)?);
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
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        Tag::parse(reader)?.validate(TagType::ID, 1)?;
        let tree_id = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(TagType::ID, 2)?;
        let node_id = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(TagType::Byte1, 3)?;
        let is_update = reader.read_bool()?;

        let subblock = SubBlock::parse(reader)?.validate_tag(4)?;
        Tag::parse(reader)?.validate(TagType::ID, 1)?;
        let parent_id = CrdtId::parse(reader)?;
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
        reader: &mut crate::Bitreader<impl Readable>,
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
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        Tag::parse(reader)?.validate(TagType::ID, 1)?;
        let block_id = CrdtId::parse(reader)?;

        Ok(RootTextBlock {
            block_id,
            text: Text::parse(reader)?,
        })
    }
}

pub struct SceneItemBlock {}
impl BlockParse for SceneItemBlock {
    fn parse(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        Tag::parse(reader)?.validate(TagType::ID, 1)?;
        let id = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(TagType::ID, 2)?;
        let item_id = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(TagType::ID, 3)?;
        let left_id = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(TagType::ID, 4)?;
        let right_id = CrdtId::parse(reader)?;
        Tag::parse(reader)?.validate(TagType::Byte4, 5)?;
        let deleted_length = CrdtId::parse(reader)?;
    }
}

#[derive(Debug)]
pub struct SceneGroupItemBlock {
    id: CrdtId,
}
impl BlockParse for SceneGroupItemBlock {
    fn parse(
        _info: &BlockInfo,
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, ParseError> {
        // XXX don't know what this means
        Tag::parse(reader)?.validate(TagType::ID, 1)?;
        let id = CrdtId::parse(reader)?;

        Ok(Self { id })
    }
}
