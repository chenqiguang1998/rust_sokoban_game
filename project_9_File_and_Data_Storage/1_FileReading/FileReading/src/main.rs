// 此文件展示了如何在 Rust 中读取文件内容
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // 尝试打开名为 "hello.txt" 的文件
    let file = File::open("hello.txt")?;
    // 创建一个基于该文件的缓冲区读取器
    let reader = BufReader::new(file);

    // 遍历读取器中的每一行
    for line in reader.lines() {
        // 打印每一行的内容
        println!("{}", line?);
    }

    Ok(())
}
