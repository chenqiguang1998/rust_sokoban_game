// 此文件展示了如何在 Rust 中写入文件内容
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // 尝试创建一个名为 "hello.txt" 的文件
    let mut file = File::create("hello.txt")?;
    // 向文件中写入字节数据 "Hello, world!"
    file.write_all(b"Hello, world!")?;
    Ok(())
}
