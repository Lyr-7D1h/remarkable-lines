use std::fs::read;

use parser::RemarkableFile;

#[test]
fn test_parsing() {
    let test_page = read("./tests/test_page.rm").unwrap();
    let rm_file = RemarkableFile::read(&test_page[..]).unwrap();

    assert!(rm_file.version() == 5)
}
