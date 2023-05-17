use std::collections::HashMap;

use crate::{
    bitreader::Readable,
    v6::Block,
    v6::{tagged_bit_reader::TaggedBitreader, TypeParse},
    Bitreader, Parse, ParseError,
};

struct Node {
    id: String,
    children: Vec<Node>,
    is_layer: bool,
    layer: i32,
    // value: *SceneTreeNode
}

pub struct SceneTree {
    nodes: HashMap<String, Node>,
    root: Node,
}

impl SceneTree {
    pub fn build_tree(blocks: Vec<Block>) -> Result<SceneTree, ParseError> {
        todo!()
    }
}

impl Parse for SceneTree {
    fn parse(
        version: u32,
        reader: &mut Bitreader<impl Readable>,
    ) -> Result<Self, crate::ParseError> {
        let mut blocks = vec![];
        let mut tagged_bit_reader = TaggedBitreader::new(reader);

        loop {
            if tagged_bit_reader.bit_reader.eof()? {
                break;
            }
            blocks.push(Block::parse(&mut tagged_bit_reader)?);
        }

        Self::build_tree(blocks)
    }
}
