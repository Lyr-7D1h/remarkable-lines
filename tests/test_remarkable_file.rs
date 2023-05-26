use std::fs::read;

use parser::RemarkableFile;

// #[test]
// fn test_v5_advent_of_code() {
//     let test_page = read("./tests/test_v5_advent_of_code.rm").unwrap();
//     let rm_file = RemarkableFile::read(&test_page[..]).unwrap();
//
//     assert!(rm_file.version() == 5);
//     match rm_file {
//         RemarkableFile::V6 { .. } => panic!("invalid version"),
//         RemarkableFile::Other { pages, .. } => {
//             assert!(pages.len() == 1);
//             let layers = &pages[0].layers;
//             assert!(layers.len() == 1);
//             assert!(layers[0].lines.len() == 139);
//         }
//     }
// }

// AuthorIdsBlock(author_uuids={1: UUID("495ba59f-c943-2b5c-b455-3682f6948906")}),
// MigrationInfoBlock(migration_id=CrdtId(1, 1), is_device=True),
// PageInfoBlock(
//     loads_count=1, merges_count=0, text_chars_count=3, text_lines_count=1
// ),
// SceneTreeBlock(
//     tree_id=CrdtId(0, 11),
//     node_id=CrdtId(0, 0),
//     is_update=True,
//     parent_id=CrdtId(0, 1),
// ),
// RootTextBlock(
//     block_id=CrdtId(0, 0),
//     value=si.Text(
//         items=CrdtSequence([
//             CrdtSequenceItem(
//                 item_id=CrdtId(1, 16),
//                 left_id=CrdtId(0, 0),
//                 right_id=CrdtId(0, 0),
//                 deleted_length=0,
//                 value="AB",
//             )
//         ]),
//         styles={
//             CrdtId(0, 0): LwwValue(timestamp=CrdtId(1, 15), value=si.ParagraphStyle.PLAIN),
//         },
//         pos_x=-468.0,
//         pos_y=234.0,
//         width=936.0,
//     )
// ),
// TreeNodeBlock(
//     group=si.Group(node_id=CrdtId(0, 1)),
// ),
// TreeNodeBlock(
//     group=si.Group(
//         node_id=CrdtId(0, 11),
//         label=LwwValue(CrdtId(0, 12), "Layer 1"),
//     ),
// ),
// SceneGroupItemBlock(
//     parent_id=CrdtId(0, 1),
//     item=CrdtSequenceItem(
//         item_id=CrdtId(0, 13),
//         left_id=CrdtId(0, 0),
//         right_id=CrdtId(0, 0),
//         deleted_length=0,
//         value=CrdtId(0, 11),
//     )
// ),

#[test]
fn test_v6_single_page_line() {
    let test_page = read("./tests/test_v6_normal_ab.rm").unwrap();
    let rm_file = RemarkableFile::read(&test_page[..]).unwrap();

    assert!(rm_file.version() == 6);
    match rm_file {
        RemarkableFile::Other { .. } => panic!("invalid version"),
        RemarkableFile::V6 { tree, blocks } => {}
    }
}
