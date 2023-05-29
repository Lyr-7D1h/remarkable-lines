#![feature(assert_matches)]
use std::{assert_matches::assert_matches, collections::HashMap, fs::read};

use remarkable_lines::{
    v6::{
        blocks::{
            AuthorsIdsBlock, MigrationInfoBlock, PageInfoBlock, RootTextBlock, SceneItemBlock,
            SceneTreeBlock, TreeNodeBlock,
        },
        crdt::{CrdtId, CrdtSequence},
        scene_item::text::Text,
        Block,
    },
    RemarkableFile,
};

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
            let mut authors = vec![(1, "495ba59f-c943-2b5c-b455-3682f6948906".to_owned())]
                .into_iter()
                .collect::<HashMap<u16, String>>();

            assert_matches!(
                blocks[..],
                [
                    Block::AuthorsIds(AuthorsIdsBlock {
                        // authors: vec![(1, "495ba59f-c943-2b5c-b455-3682f6948906".to_owned())]
                        //     .into_iter()
                        //     .collect::<HashMap<u16, String>>()
                        ..
                    }),
                    Block::MigrationInfo(MigrationInfoBlock {
                        migration_id: CrdtId { part1: 1, part2: 1 },
                        is_device: true
                    }),
                    Block::PageInfo(PageInfoBlock {
                        loads_count: 1,
                        merges_count: 0,
                        text_chars_count: 3,
                        text_lines_count: 1
                    }),
                    Block::SceneTree(SceneTreeBlock {
                        tree_id: CrdtId {
                            part1: 0,
                            part2: 11
                        },
                        node_id: CrdtId { part1: 0, part2: 0 },
                        is_update: true,
                        parent_id: CrdtId { part1: 0, part2: 1 }
                    }),
                    Block::RootText(RootTextBlock {
                        block_id: CrdtId { part1: 0, part2: 0 },
                        text: Text {
                            items: CrdtSequence::new(),
                            styles,
                            x: -468.0,
                            y: 234.0,
                            width: 936.0
                        },
                    }),
                    Block::TreeNode(TreeNodeBlock { .. }),
                    Block::TreeNode(TreeNodeBlock { .. }),
                    Block::SceneGroupItem(SceneItemBlock { .. }),
                ]
            )
        }
    }
}
