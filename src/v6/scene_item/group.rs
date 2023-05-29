use crate::v6::{
    crdt::{CrdtId, CrdtSequence},
    lwwvalue::LwwValue,
};

use super::SceneItem;

/// A Group represents a group of nested items.
///
/// Groups are used to represent layers.
///
/// node_id is the id that this sub-tree is stored as a "SceneTreeBlock".
//
/// children is a sequence of other SceneItems.
///
/// `anchor_id` refers to a text character which provides the anchor y-position
/// for this group. There are two values that seem to be special:
/// - `0xfffffffffffe` seems to be used for lines right at the top of the page?
/// - `0xffffffffffff` seems to be used for lines right at the bottom of the page?
#[derive(Debug, Clone)]
pub struct Group {
    pub node_id: CrdtId,
    pub children: CrdtSequence<SceneItem>, // = field(default_factory=CrdtSequence),
    pub label: LwwValue<String>,
    pub visible: LwwValue<bool>, //LwwValue(CrdtId(0, 0), True),

    pub anchor_id: Option<LwwValue<CrdtId>>,
    pub anchor_type: Option<LwwValue<u8>>,
    pub anchor_threshold: Option<LwwValue<f32>>,
    pub anchor_origin_x: Option<LwwValue<f32>>,
}

impl Group {
    pub fn node_id(mut self, node_id: CrdtId) -> Self {
        self.node_id = node_id;
        self
    }
    pub fn label(mut self, label: LwwValue<String>) -> Self {
        self.label = label;
        self
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            node_id: Default::default(),
            children: Default::default(),
            label: LwwValue {
                timestamp: CrdtId::default(),
                value: String::new(),
            },
            visible: LwwValue {
                timestamp: CrdtId::default(),
                value: true,
            },
            anchor_id: None,
            anchor_type: None,
            anchor_threshold: None,
            anchor_origin_x: None,
        }
    }
}
