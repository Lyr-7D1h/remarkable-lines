use std::{collections::HashMap, fs::read, hash::Hash};

use remarkable_lines::{
    v6::{
        blocks::{
            AuthorsIdsBlock, MigrationInfoBlock, PageInfoBlock, RootTextBlock, SceneItemBlock,
            SceneTreeBlock, TreeNodeBlock,
        },
        crdt::{CrdtId, CrdtSequenceItem},
        lwwvalue::LwwValue,
        scene_item::{
            group::Group,
            text::{ParagraphStyle, Text, TextItem},
        },
        Block,
    },
    RemarkableFile,
};

fn vec_to_hashmap<K: Eq + Hash, V>(items: Vec<(K, V)>) -> HashMap<K, V> {
    return items.into_iter().collect();
}

#[test]
fn test_v5_advent_of_code() {
    let test_page = read("./tests/fixtures/test_v5_advent_of_code.rm").unwrap();
    let rm_file = RemarkableFile::read(&test_page[..]).unwrap();

    assert!(rm_file.version() == 5);
    match rm_file {
        RemarkableFile::V6 { .. } => panic!("invalid version"),
        RemarkableFile::Other { pages, .. } => {
            assert!(pages.len() == 1);
            let layers = &pages[0].layers;
            assert!(layers.len() == 1);
            assert!(layers[0].lines.len() == 139);
        }
    }
}

#[test]
fn test_v6_single_page_line() {
    let test_page = read("./tests/fixtures/test_v6_normal_ab.rm").unwrap();
    // let test_page = read("./tests/fixtures/test_v6_single_page_line.rm").unwrap();
    let rm_file = RemarkableFile::read(&test_page[..]).unwrap();

    assert!(rm_file.version() == 6);
    println!("{rm_file:?}");
    match rm_file {
        RemarkableFile::Other { .. } => panic!("invalid version"),
        RemarkableFile::V6 { tree, blocks } => {
            println!("{tree:?}");
            let expected_blocks = vec![
                Block::AuthorsIds(AuthorsIdsBlock {
                    authors: vec_to_hashmap(vec![(
                        1,
                        "495ba59f-c943-2b5c-b455-3682f6948906".to_owned(),
                    )]),
                }),
                Block::MigrationInfo(MigrationInfoBlock {
                    migration_id: CrdtId { part1: 1, part2: 1 },
                    is_device: true,
                }),
                Block::PageInfo(PageInfoBlock {
                    loads_count: 1,
                    merges_count: 0,
                    text_chars_count: 3,
                    text_lines_count: 1,
                }),
                Block::SceneTree(SceneTreeBlock {
                    tree_id: CrdtId {
                        part1: 0,
                        part2: 11,
                    },
                    node_id: CrdtId { part1: 0, part2: 0 },
                    is_update: true,
                    parent_id: CrdtId { part1: 0, part2: 1 },
                }),
                Block::RootText(RootTextBlock {
                    block_id: CrdtId { part1: 0, part2: 0 },
                    text: Text {
                        x: -468.0,
                        y: 234.0,
                        width: 936.0,
                        items: vec![CrdtSequenceItem {
                            item_id: CrdtId {
                                part1: 1,
                                part2: 16,
                            },
                            left_id: CrdtId { part1: 0, part2: 0 },
                            right_id: CrdtId { part1: 0, part2: 0 },
                            deleted_length: 0,
                            value: TextItem::Text(String::from("AB")),
                        }]
                        .into_iter()
                        .collect(),
                        styles: vec_to_hashmap(vec![(
                            CrdtId { part1: 0, part2: 0 },
                            LwwValue {
                                timestamp: CrdtId {
                                    part1: 1,
                                    part2: 15,
                                },
                                value: ParagraphStyle::PLAIN,
                            },
                        )]),
                    },
                }),
                Block::TreeNode(TreeNodeBlock {
                    group: Group::default().node_id(CrdtId { part1: 0, part2: 1 }),
                }),
                Block::TreeNode(TreeNodeBlock {
                    group: Group::default()
                        .node_id(CrdtId {
                            part1: 0,
                            part2: 11,
                        })
                        .label(LwwValue {
                            timestamp: CrdtId {
                                part1: 0,
                                part2: 12,
                            },
                            value: String::from("Layer 1"),
                        }),
                }),
                Block::SceneGroupItem(SceneItemBlock {
                    parent_id: CrdtId { part1: 0, part2: 1 },
                    item: CrdtSequenceItem {
                        item_id: CrdtId {
                            part1: 0,
                            part2: 13,
                        },
                        left_id: CrdtId { part1: 0, part2: 0 },
                        right_id: CrdtId { part1: 0, part2: 0 },
                        deleted_length: 0,
                        value: Some(CrdtId {
                            part1: 0,
                            part2: 11,
                        }),
                    },
                }),
            ];

            assert_eq!(format!("{blocks:?}"), format!("{expected_blocks:?}"));
        }
    }
}
