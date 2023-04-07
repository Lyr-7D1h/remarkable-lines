use std::{collections::HashMap, io::Read};

use crate::ParseError;

use super::{BlockInfo, BlockParse};

#[derive(Debug)]
pub struct MigrationInfoBlock {}
impl BlockParse for MigrationInfoBlock {
    fn parse<N: Read>(
        info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct AuthorsIdsBlock {
    authors: HashMap<u16, String>,
}
impl BlockParse for AuthorsIdsBlock {
    fn parse<N: Read>(
        _info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        let amount_subblocks = reader.read_varunit()?;
        let mut authors = HashMap::new();

        println!("A {amount_subblocks}");
        for _ in 0..amount_subblocks {
            let tag = reader.read_varunit()?;
            let subblock_length = reader.read_u32()?;
            let uuid_length = reader.read_varunit()?;
            if uuid_length != 16 {
                return Err(ParseError::invalid("Expected UUID length to be 16 bytes"));
            }
            let uuid = String::from_utf8(reader.read_bytes(uuid_length as usize)?)?;
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
        info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct TreeNodeBlock {}
impl BlockParse for TreeNodeBlock {
    fn parse<N: Read>(
        info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}
#[derive(Debug)]
pub struct SceneTreeBlock {}
impl BlockParse for SceneTreeBlock {
    fn parse<N: Read>(
        info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}
#[derive(Debug)]
pub struct SceneGlyphItem {}
impl BlockParse for SceneGlyphItem {
    fn parse<N: Read>(
        info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}
