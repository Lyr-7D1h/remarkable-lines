use std::collections::HashMap;

use crate::{
    bitreader::Readable,
    v6::Block,
    v6::{tagged_bit_reader::TaggedBitreader, TypeParse},
    Bitreader, Parse, ParseError,
};

use super::{
    crdt::{CrdtId, CrdtSequenceItem},
    scene_item::{group::Group, text::Text, SceneItem},
};

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
        group.node_id = id.clone();
        self.nodes.insert(id, group);
    }

    pub fn add_item(
        &mut self,
        item: CrdtSequenceItem<SceneItem>,
        parent_id: CrdtId,
    ) -> Result<(), ParseError> {
        let parent = self
            .nodes
            .get_mut(&parent_id)
            .ok_or(ParseError::invalid(format!(
                "Could not find parent: {parent_id:?}"
            )))?;
        parent.children.push(item);
        return Ok(());
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
                        children: node.children.clone(),
                        ..b.group
                    };
                }
                Block::SceneGroupItem(b) => {
                    match &b.item.value {
                        Some(v) => {
                            let node = tree.nodes.get(&v).ok_or(ParseError::invalid(format!(
                                "Node does not exist for ScneGroupItemBlock: {v:?}"
                            )))?;
                            let block_item = b.item;
                            let item = CrdtSequenceItem {
                                value: SceneItem::Group(node.clone()),
                                item_id: block_item.item_id,
                                left_id: block_item.left_id,
                                right_id: block_item.right_id,
                                deleted_length: block_item.deleted_length,
                            };
                            tree.add_item(item, b.parent_id)?;
                        }
                        None => {
                            return Err(ParseError::invalid(format!("No node id found",)));
                        }
                    };
                }
                Block::SceneGlyphItem(b) => {
                    if let Some(glyph) = b.item.value {
                        // let mut block_item = b.item;
                        let item = CrdtSequenceItem {
                            value: SceneItem::GlyphRange(glyph),
                            item_id: b.item.item_id,
                            left_id: b.item.left_id,
                            right_id: b.item.right_id,
                            deleted_length: b.item.deleted_length,
                        };

                        tree.add_item(item, b.parent_id)?;
                    }
                }
                Block::SceneLineItem(b) => {
                    if let Some(line) = b.item.value {
                        // let mut block_item = b.item;
                        let item = CrdtSequenceItem {
                            value: SceneItem::Line(line),
                            item_id: b.item.item_id,
                            left_id: b.item.left_id,
                            right_id: b.item.right_id,
                            deleted_length: b.item.deleted_length,
                        };

                        tree.add_item(item, b.parent_id)?;
                    }
                }
                Block::RootText(b) => tree.root_text = Some(b.text),
                _ => (),
            }
        }

        Ok(tree)
    }
}

impl Parse for SceneTree {
    fn parse(
        _version: u32,
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
