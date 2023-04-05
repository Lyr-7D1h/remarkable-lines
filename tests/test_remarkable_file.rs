use std::fs::read;

use parser::RemarkableFile;

#[test]
fn test_v5_advent_of_code() {
    let test_page = read("./tests/test_v5_advent_of_code.rm").unwrap();
    let rm_file = RemarkableFile::read(&test_page[..]).unwrap();

    assert!(rm_file.version() == 5);
    let pages = rm_file.pages();
    assert!(pages.len() == 1);
    let layers = &pages[0].layers;
    assert!(layers.len() == 1);
    assert!(layers[0].lines.len() == 139);
}

// #[test]
// fn test_v6_single_page_line() {
//     let test_page = read("./tests/test_v6_single_page_line.rm").unwrap();
//     let rm_file = RemarkableFile::read(&test_page[..]).unwrap();
//
//     assert!(rm_file.version() == 6);
//     let pages = rm_file.pages();
//     assert!(pages.len() == 1);
//     let layers = &pages[0].layers;
//     assert!(layers.len() == 1);
//     assert!(layers[0].lines.len() == 139);
// }
