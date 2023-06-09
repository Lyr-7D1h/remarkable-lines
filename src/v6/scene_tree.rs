use std::collections::HashMap;

use crate::ParseError;

use super::{
    block::Block,
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

#[derive(Debug)]
pub struct SceneTree {
    nodes: HashMap<CrdtId, Group>,
    root_id: CrdtId,
    root_text: Option<Text>,
}

impl SceneTree {
    pub fn add_node(&mut self, id: CrdtId) {
        let mut group = Group::default();
        group.node_id = id.clone();
        self.nodes.insert(id, group);
    }

    pub fn get_node(&mut self, id: &CrdtId) -> Option<&Group> {
        self.nodes.get(id)
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
        let root_id = CrdtId { part1: 0, part2: 1 };
        let mut nodes = HashMap::new();
        let mut root = Group::default();
        root.node_id = root_id;
        nodes.insert(root_id, root);
        SceneTree {
            root_id,
            nodes,
            root_text: None,
        }
    }

    pub fn from_blocks(blocks: &Vec<Block>) -> Result<SceneTree, ParseError> {
        let mut tree = SceneTree::new();
        for block in blocks.into_iter() {
            let block = block.clone();
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
