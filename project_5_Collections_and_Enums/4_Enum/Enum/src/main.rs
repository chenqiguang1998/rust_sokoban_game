// 此文件展示了 Rust 中枚举的定义和使用
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // 创建一个 Direction 类型的枚举值
    let dir = Direction::Up;

    // 使用 match 进行模式匹配并处理不同的枚举值
    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}
