 # Remarkable Lines File Parser
 This parser understands and parses the files used by the Remarkable Paper Tablet.
 These files include many things under which is lines, color and text.

> [!WARNING]
> I have not yet had the time to really test most versions. This is delivered as is. Any PR's are very welcome!
> I will revisit this project in a later stage.

 # Support
 Currently **V3 up to V6** is supported.
 Although some bugs and undefined behavior might occur as this library is not yet widely tested.
 The parser will indicate if the version is not supported.

 Some data points involve guess work as the file format is proprietery and is reverse engineered.

 # Reading a `.rm` file
 You can read any remarkble file

 ```rust
 use std::{fs::read};
 use remarkable_lines::{RemarkableFile};

 pub fn main() {
     let test_file = read("./test.rm").unwrap();
     let rm_file = RemarkableFile::read(&test_file[..]).unwrap();
     println!("{rm_file:?");
 }
 ```

# Resources used
File Format:
- https://plasma.ninja/blog/devices/remarkable/binary/format/2017/12/26/reMarkable-lines-file-format.html
- https://github.com/ax3l/lines-are-rusty
- https://docs.rs/rm-lines/0.1.0/src/rm_lines/lib.rs.html
- https://github.com/rorycl/rm2pdf
V6 Version:
- https://github.com/ricklupton/rmscene
- https://github.com/ddvk/reader
- https://www.reddit.com/r/RemarkableTablet/comments/10hxe3j/updates_regarding_reverse_engineering_remarkable/
