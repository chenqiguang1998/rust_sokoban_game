
fn main() {
    // if 条件语句示例
    let number = 5;

    if number % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }

    // for 循环示例
    for i in 1..5 {
        println!("i: {}", i);
    }

    // while 循环示例
    let mut number = 3;
    while number!= 0 {
        println!("number: {}", number);
        number -= 1;
    }

    // loop 循环示例
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!("Count reached: {}", count);

    // match 模式匹配示例
    let number = 5;

    match number {
        1 => println!("One"),
        2 | 3 | 5 => println!("Prime"),
        4..=10 => println!("Between 4 and 10"),
        _ => println!("Something else"),
    }
}

