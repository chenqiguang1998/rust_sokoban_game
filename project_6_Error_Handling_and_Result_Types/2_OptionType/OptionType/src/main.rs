// 主函数，用于演示 Option 类型的使用
fn main() {
    // 创建一个包含值 "Alice" 的 Some 类型变量
    let name = Some("Alice");
    // 使用 match 语句来处理 Option 的两种可能情况：Some 和 None
    match name {
        // 如果是 Some 且包含值 n，打印出名字
        Some(n) => println!("Name: {}", n),
        // 如果是 None，打印 "No name"
        None => println!("No name"),
    }

    // 创建一个值为 None 的 Option<&str> 类型变量
    let absent_name: Option<&str> = None;
    // 使用 if let 语句来简洁地处理 Option 的值
    if let Some(n) = absent_name {
        // 如果 Option 包含值，打印出名字
        println!("Name: {}", n);
    } else {
        // 如果 Option 不包含值，打印 "No name"
        println!("No name");
    }
}
