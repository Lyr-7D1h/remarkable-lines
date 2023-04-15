use std::{collections::HashMap, fmt::format, io::Read};

use crate::{
    v6::block::{
        tag::{Tag, TagType},
        TypeParse,
    },
    ParseError,
};

use super::{crdtid::CrdtId, BlockInfo, BlockParse};

#[derive(Debug)]
pub struct MigrationInfoBlock {
    migration_id: CrdtId,
    is_device: bool,
}
impl BlockParse for MigrationInfoBlock {
    fn parse<N: Read>(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        let migration_id = CrdtId::parse(reader)?;

        let tag = Tag::parse(reader)?;
        if tag.tag_type != TagType::Byte1 {
            return Err(ParseError::invalid("invalid tagtype given"));
        }
        let is_device = reader.read_u8()? > 0;

        println!("{is_device}");
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
    fn parse<N: Read>(
        _info: &BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        let amount_subblocks = reader.read_varuint()?;
        let mut authors = HashMap::new();

        for _ in 0..amount_subblocks {
            let tag = Tag::parse(reader)?;
            if tag.tag_type != TagType::Length4 {
                return Err(ParseError::invalid(format!(
                    "Invalid tag type received {:?} expected {:?}",
                    tag.tag_type,
                    TagType::Length4
                )));
            }

            let _subblock_length = reader.read_u32()?;

            let uuid = reader.read_uuid()?;

            let author_id = reader.read_u16()?;
            authors.insert(author_id, uuid);
        }

        Ok(Self { authors })
    }
}

#[derive(Debug)]
pub struct PageInfoBlock {}
impl BlockParse for PageInfoBlock {
    fn parse<N: Read>(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct TreeNodeBlock {}
impl BlockParse for TreeNodeBlock {
    fn parse<N: Read>(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}
#[derive(Debug)]
pub struct SceneTreeBlock {}
impl BlockParse for SceneTreeBlock {
    fn parse<N: Read>(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}
#[derive(Debug)]
pub struct SceneGlyphItem {}
impl BlockParse for SceneGlyphItem {
    fn parse<N: Read>(
        info: &BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}
