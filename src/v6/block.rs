use std::io::Read;

use crate::{Parse, ParseError};

#[derive(Debug)]
pub struct BlockInfo {
    // offset: u32,
    size: u32,
    min_version: u8,
    current_version: u8,
}

#[derive(Debug)]
struct MigrationInfoBlock {}
#[derive(Debug)]
struct PageInfoBlock {}
#[derive(Debug)]
struct TreeNodeBlock {}
#[derive(Debug)]
struct SceneTreeBlock {}
#[derive(Debug)]
struct SceneGlyphItem {}
#[derive(Debug)]
struct IDKBLock {}

#[derive(Debug)]
pub enum Block {
    MigrationInfo(MigrationInfoBlock),
    PageInfo(PageInfoBlock),
    TreeNode(TreeNodeBlock),
    SceneTree,
    SceneGlyphItem,
    SceneGroupItem,
    SceneLineItem,
    SceneTextItem,
    AuthorsIds(AuthorsIds),
    RootText,
}

#[derive(Debug)]
pub struct AuthorsIds {}

impl BlockParse for AuthorsIds {
    fn parse<N: Read>(
        info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        Ok(AuthorsIds {})
    }
}

pub trait BlockParse {
    fn parse<N: Read>(
        info: BlockInfo,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl Parse for Block {
    fn parse<N: std::io::Read>(
        version: u32,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, ParseError> {
        let size = reader.read_u32()?;
        let _ = reader.read_u8()?;
        let min_version = reader.read_u8()?;
        let current_version = reader.read_u8()?;
        let block_type = reader.read_u8()?;

        let info = BlockInfo {
            size,
            min_version,
            current_version,
        };

        Ok(match block_type {
            0x00 => Block::MigrationInfo,
            0x01 => Block::SceneTree,
            0x02 => Block::TreeNode,
            0x03 => Block::SceneGlyphItem,
            0x04 => Block::SceneGroupItem,
            0x05 => Block::SceneLineItem,
            0x06 => Block::SceneTextItem,
            0x07 => Block::RootText,
            0x09 => Block::AuthorsIds(AuthorsIds::parse(info, reader)?),
            0x0A => Block::PageInfo,
            _ => {
                return Err(ParseError::invalid(format!(
                    "Unknown block type: '{block_type}'"
                )))
            }
        })
    }
}
