use super::{text::LwwValue, crdtid::CrdtId};

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
pub struct Group {
    node_id: CrdtId,
    children: CrdtSequence[SceneItem] = field(default_factory=CrdtSequence),
    label: LwwValue[str] = LwwValue(CrdtId(0, 0), ""),
    visible: LwwValue[bool] = LwwValue(CrdtId(0, 0), True),

    anchor_id: tp.Optional[LwwValue[CrdtId]] = None,
    anchor_type: tp.Optional[LwwValue[int]] = None,
    anchor_threshold: tp.Optional[LwwValue[float]] = None,
    anchor_origin_x: tp.Optional[LwwValue[float]] = None,
}

impl TypeParse for Group {

}