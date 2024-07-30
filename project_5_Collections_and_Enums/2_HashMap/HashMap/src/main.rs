// 此文件展示了 Rust 中 HashMap 的基本用法
use std::collections::HashMap;

fn main() {
    // 创建一个空的 HashMap
    let mut scores = HashMap::new();

    // 向 HashMap 中插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 获取指定键对应的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // 根据获取结果进行处理
    match score {
        Some(&score) => println!("Score: {}", score),
        None => println!("No score found."),
    }

    // 遍历 HashMap 并打印键值对
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
