// 定义一个加法函数
fn add(a: i32, b: i32) -> i32 {
    // 执行加法操作并返回结果
    a + b
}

// 定义一个问候函数
fn greet(name: &str) -> String {
    // 使用 format! 宏生成问候字符串并返回
    format!("Hello, {}!", name)
}

// 定义一个获取较长字符串的函数，并带有生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        // 如果 x 的长度大于 y 的长度，返回 x
        x
    } else {
        // 否则返回 y
        y
    }
}

fn main() {
    // 调用 add 函数计算 5 和 3 的和，并将结果存储在 result 变量中
    let result = add(5, 3);
    // 打印计算结果
    println!("Result: {}", result);

    // 调用 greet 函数，传入 "Alice" 作为参数，并将返回的问候字符串存储在 greeting 变量中
    let greeting = greet("Alice");
    // 打印问候字符串
    println!("{}", greeting);

    // 定义两个字符串 str1 和 str2
    let str1 = "long string";
    let str2 = "short";
    // 调用 longest 函数获取较长的字符串，并将结果存储在 result 变量中
    let result = longest(str1, str2);
    // 打印较长的字符串
    println!("Longest string: {}", result);
}

