use std::collections::HashMap;

use crate::{
    bitreader::Readable,
    v6::Block,
    v6::{tagged_bit_reader::TaggedBitreader, TypeParse},
    Bitreader, Parse, ParseError,
};

use super::{crdt::CrdtId, group::Group, text::Text};

struct Node {
    id: CrdtId,
    children: Vec<Node>,
    is_layer: bool,
    layer: i32,
    // value: *SceneTreeNode
}

pub struct SceneTree {
    nodes: HashMap<CrdtId, Group>,
    root: Group,
    root_text: Option<Text>,
}

impl SceneTree {
    pub fn add_node(&mut self, id: CrdtId) {
        let mut group = Group::default();
        group.node_id = id;
        self.nodes.insert(id, group);
    }

    pub fn new() -> SceneTree {
        let mut root = Group::default();
        root.node_id = CrdtId { part1: 0, part2: 1 };
        SceneTree {
            root,
            nodes: HashMap::new(),
            root_text: None,
        }
    }

    pub fn from_blocks(blocks: Vec<Block>) -> Result<SceneTree, ParseError> {
        let mut tree = SceneTree::new();
        for block in blocks.into_iter() {
            match block {
                Block::SceneTree(b) => {
                    // XXX check node_id and is_update
                    // pending_tree_nodes[b.tree_id] = b
                    tree.add_node(b.tree_id)
                }
                Block::TreeNode(b) => {
                    let node = match tree.nodes.get_mut(&b.group.node_id) {
                        Some(node) => node,
                        None => {
                            return Err(ParseError::invalid(format!(
                                "Node does not exist for TreeNodeBlock: {:?}",
                                b.group.node_id
                            )));
                        }
                    };
                    *node = Group {
                        children: node.children,
                        ..b.group
                    };
                }
                Block::SceneGroupItem(_) => todo!(),
                Block::SceneGlyphItem | Block::SceneLineItem => todo!(),
                Block::RootText(_) => todo!(),
                _ => (),
            }
        }

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

        Self::from_blocks(blocks)
    }
}
