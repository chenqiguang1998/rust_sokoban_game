fn main() {
    // 整数、浮点数、布尔值和字符
    let a: i32 = 10; // 32 位有符号整数
    let b: f64 = 20.5; // 64 位浮点数
    let c: bool = true; // 布尔值
    let d: char = 'R'; // 字符
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    // 数组与切片
    let arr: [i32; 3] = [1, 2, 3]; // 数组
    let slice: &[i32] = &arr[1..3]; // 切片
    println!("arr: {:?}, slice: {:?}", arr, slice);

    // 元组
    let tuple: (i32, f64, char) = (10, 20.5, 'R');
    println!("tuple: {:?}", tuple);

    // 结构体
    struct Player {
        name: String,
        score: u32,
    }

    let player = Player {
        name: String::from("Alice"),
        score: 50,
    };
    println!("Player: {}, Score: {}", player.name, player.score);
}

