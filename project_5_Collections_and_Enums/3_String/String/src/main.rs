// 此文件展示了 Rust 中 String 的基本用法
fn main() {
    // 创建一个可变的 String 并初始化为 "Hello"
    let mut s = String::from("Hello");

    // 向 String 中添加内容
    s.push_str(", world!");

    // 打印 String
    println!("{}", s);

    // 创建多个 String 并通过 format! 宏组合
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // 打印组合后的 String
    println!("{}", s);
}
