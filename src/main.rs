extern crate tar;

use std::fs::File;

fn main() {
    let file = File::create("foo.tar").unwrap();
    let mut builder = tar::Builder::new(file);
    builder.append_dir_all("foo", "foo").unwrap();
    builder.finish().unwrap();
}
