use std::collections::HashMap;

use crate::{
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
    fn parse<N: std::io::Read>(
        version: u32,
        reader: &mut crate::Bitreader<N>,
    ) -> Result<Self, crate::ParseError> {
        loop {
            let block = Block::parse(reader)?;
            println!("{block:?}");
        }

        todo!()
    }
}
