use std::collections::HashMap;

use crate::{
    bitreader::Readable,
    v6::Block,
    v6::{tagged_bit_reader::TaggedBitreader, TypeParse},
    Bitreader, Parse,
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

impl Parse for SceneTree {
    fn parse(
        version: u32,
        reader: &mut Bitreader<impl Readable>,
    ) -> Result<Self, crate::ParseError> {
        let mut tagged_bit_reader = TaggedBitreader::new(reader);
        loop {
            let block = Block::parse(&mut tagged_bit_reader)?;
            println!("{block:?}");
        }

        todo!()
    }
}
