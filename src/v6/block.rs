use crate::{bitreader::Readable, Bitreader, ParseError};

mod blocks;
use blocks::*;

use super::{tagged_bit_reader::TaggedBitreader, TypeParse};

#[derive(Debug)]
pub struct BlockInfo {
    start_offset: u64,
    size: u32,
    min_version: u8,
    current_version: u8,
}

impl BlockInfo {
    pub fn has_bytes_remaining(&self, reader: &Bitreader<impl Readable>) -> bool {
        self.start_offset + self.size as u64 > reader.position()
    }
}

#[derive(Debug)]
pub enum Block {
    MigrationInfo(MigrationInfoBlock),
    PageInfo(PageInfoBlock),
    TreeNode(TreeNodeBlock),
    SceneTree(SceneTreeBlock),
    SceneGlyphItem,
    SceneGroupItem(SceneGroupItemBlock),
    SceneLineItem,
    SceneTextItem,
    AuthorsIds(AuthorsIdsBlock),
    RootText(RootTextBlock),
}

/// Parsing methods for parsing blocks
pub trait BlockParse {
    fn parse(
        info: &BlockInfo,
        reader: &mut TaggedBitreader<impl Readable>,
    ) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl TypeParse for Block {
    fn parse(reader: &mut TaggedBitreader<impl Readable>) -> Result<Self, ParseError> {
        let size = reader.bit_reader.read_u32()?;

        // unknown value
        let _ = reader.bit_reader.read_u8()?;
        let min_version = reader.bit_reader.read_u8()?;
        let current_version = reader.bit_reader.read_u8()?;
        let block_type = reader.bit_reader.read_u8()?;

        if current_version < min_version {
            return Err(ParseError::invalid(
                "current_version can't be smaller than min_version",
            ));
        }

        let start_offset = reader.bit_reader.position();

        println!(
            "\nStarting new block at offset {:x} until {:x}",
            reader.bit_reader.position() - 4,
            start_offset + size as u64
        );

        let info = BlockInfo {
            start_offset,
            size,
            min_version,
            current_version,
        };

        let block = match block_type {
            0x00 => Block::MigrationInfo(MigrationInfoBlock::parse(&info, reader)?),
            0x01 => Block::SceneTree(SceneTreeBlock::parse(&info, reader)?),
            0x02 => Block::TreeNode(TreeNodeBlock::parse(&info, reader)?),
            0x03 => Block::SceneGlyphItem,
            0x04 => Block::SceneGroupItem(SceneGroupItemBlock::parse(&info, reader)?),
            0x05 => Block::SceneLineItem,
            0x06 => Block::SceneTextItem,
            0x07 => Block::RootText(RootTextBlock::parse(&info, reader)?),
            0x09 => Block::AuthorsIds(AuthorsIdsBlock::parse(&info, reader)?),
            0x0A => Block::PageInfo(PageInfoBlock::parse(&info, reader)?),
            _ => {
                return Err(ParseError::invalid(format!(
                    "Unknown block type: '{block_type}'"
                )))
            }
        };

        let expected_offset = start_offset + size as u64;
        let end_offset = reader.bit_reader.position();
        if expected_offset != end_offset {
            return Err(ParseError::invalid(format!(
                "Block type '{block_type}' did not read expected size. got {end_offset:x} expected {expected_offset:x}" 
            )));
        }

        return Ok(block);
    }
}
