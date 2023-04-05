use std::fs::read;

use parser::RemarkableFile;

#[test]
fn test_parsing() {
    let test_page = read("./tests/test_v5_advent_of_code.rm").unwrap();
    let rm_file = RemarkableFile::read(&test_page[..]).unwrap();

    assert!(rm_file.version() == 5);
    let pages = rm_file.pages();
    assert!(pages.len() == 1);
    let layers = &pages[0].layers;
    assert!(layers.len() == 1);
    println!("{:?}", layers[0].lines)
}
