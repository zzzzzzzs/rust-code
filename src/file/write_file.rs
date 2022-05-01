// 写入文件

use std::{fs::File, io::Write};

// 写入文件
fn write_file() {
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
}

fn main() {
    write_file();
}
