// 主函数，用于演示 Result 类型的使用
fn main() {
    // 调用 divide 函数并获取其返回的 Result 值
    let result = divide(10, 2);
    // 使用 match 语句来处理 Result 的两种可能情况：Ok 和 Err
    match result {
        // 如果结果是成功（Ok），打印出结果值
        Ok(value) => println!("Result: {}", value),
        // 如果结果是失败（Err），打印出错误信息
        Err(e) => println!("Error: {}", e),
    }
}

// 定义一个执行除法运算的函数，如果除数为 0 则返回错误
fn divide(a: i32, b: i32) -> Result<i32, String> {
    // 如果除数为 0，返回一个错误结果
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        // 否则，返回除法运算的结果，作为成功结果
        Ok(a / b)
    }
}
