#![feature(assert_matches)]
use std::{assert_matches::assert_matches, collections::HashMap, fs::read};

use remarkable_lines::{
    v6::{
        blocks::{
            AuthorsIdsBlock, MigrationInfoBlock, PageInfoBlock, RootTextBlock, SceneItemBlock,
            SceneTreeBlock, TreeNodeBlock,
        },
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
            let mut authors = HashMap::new();
            authors.insert(1, String::from("495ba59f-c943-2b5c-b455-3682f6948906"));

            assert_matches!(
                blocks[..],
                [
                    Block::AuthorsIds(AuthorsIdsBlock { .. }),
                    Block::MigrationInfo(MigrationInfoBlock { .. }),
                    Block::PageInfo(PageInfoBlock { .. }),
                    Block::SceneTree(SceneTreeBlock { .. }),
                    Block::RootText(RootTextBlock { .. }),
                    Block::TreeNode(TreeNodeBlock { .. }),
                    Block::TreeNode(TreeNodeBlock { .. }),
                    Block::SceneGroupItem(SceneItemBlock { .. }),
                ]
            )
        }
    }
}
