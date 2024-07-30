ifn main() {
    // 定义和使用变量
    let x = 5; // 不可变变量
    let mut y = 10; // 可变变量
    y += 5; // 修改可变变量的值
    println!("x: {}, y: {}", x, y);

    // 常量与不可变变量
    const MAX_SCORE: u32 = 100;
    println!("Max Score: {}", MAX_SCORE);

    // 变量的作用域和生命周期
    {
        let z = 20; // 变量 z 的作用域在这个代码块内
        println!("z: {}", z);
    }
    // println!("z: {}", z); // 这行代码会报错，因为 z 已经超出作用域
}

