use std::io::Read;

use crate::{Parse, ParseError};

mod blocks;
use blocks::*;

#[derive(Debug)]
pub struct BlockInfo {
    // offset: u32,
    size: u32,
    min_version: u8,
    current_version: u8,
}

#[derive(Debug)]
pub enum Block {
    MigrationInfo(MigrationInfoBlock),
    PageInfo(PageInfoBlock),
    TreeNode(TreeNodeBlock),
    SceneTree(SceneTreeBlock),
    SceneGlyphItem,
    SceneGroupItem,
    SceneLineItem,
    SceneTextItem,
    AuthorsIds(AuthorsIdsBlock),
    RootText,
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
        println!("Starting new block at offset {:x}", reader.offset());
        let start_offset = reader.offset();

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

        let block = match block_type {
            0x00 => Block::MigrationInfo(MigrationInfoBlock::parse(info, reader)?),
            0x01 => Block::SceneTree(SceneTreeBlock::parse(info, reader)?),
            0x02 => Block::TreeNode(TreeNodeBlock::parse(info, reader)?),
            0x03 => Block::SceneGlyphItem,
            0x04 => Block::SceneGroupItem,
            0x05 => Block::SceneLineItem,
            0x06 => Block::SceneTextItem,
            0x07 => Block::RootText,
            0x09 => Block::AuthorsIds(AuthorsIdsBlock::parse(info, reader)?),
            0x0A => Block::PageInfo(PageInfoBlock::parse(info, reader)?),
            _ => {
                return Err(ParseError::invalid(format!(
                    "Unknown block type: '{block_type}'"
                )))
            }
        };

        let expected_offset = start_offset + usize::try_from(size)?;
        let end_offset = reader.offset();
        if expected_offset != reader.offset() {
            return Err(ParseError::invalid(format!(
                "Block type '{block_type}' did not read expected size. Expected {expected_offset:x} given {end_offset:x}" 
            )));
        }

        return Ok(block);
    }
}
