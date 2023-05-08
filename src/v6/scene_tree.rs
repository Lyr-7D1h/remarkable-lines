use std::collections::HashMap;

use crate::{
    bitreader::Readable,
    v6::{block::TypeParse, Block},
    Parse,
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
        reader: &mut crate::Bitreader<impl Readable>,
    ) -> Result<Self, crate::ParseError> {
        loop {
            let block = Block::parse(reader)?;
            println!("{block:?}");
        }

        todo!()
    }
}
