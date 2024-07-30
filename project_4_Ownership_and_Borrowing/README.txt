# Rust 中的所有权与借用

在这部分内容中，我们将深入研究 Rust 中的所有权和借用机制，并通过示例代码进一步完善推箱子游戏的实现。

## 所有权概念

### 所有权规则
Rust 通过独特的所有权系统来管理内存。每个值都有一个明确的所有者，并且遵循以下三个基本规则：
1. 每个值都有一个所有者。
2. 每个值在同一时间只能有一个所有者。
3. 当所有者离开作用域时，值会被自动释放。

示例代码：
```rust
fn main() {
    let s = String::from("hello");
    // s 是该字符串的所有者
    println!("{}", s);
    // s 离开作用域，字符串被释放
}
```

### 所有权的转移
所有权可以通过赋值或传递给函数进行转移。

示例代码：
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被转移到 s2
    // println!("{}", s1); // 这行代码会报错，因为 s1 不再拥有该值
    println!("{}", s2);
}
```

### 函数与所有权
函数参数的所有权在传递时也可能发生转移。

示例代码：
```rust
fn main() {
    let s = String::from("hello");
    take_ownership(s); // s 的所有权被转移到函数
    // println!("{}", s); // 这行代码会报错，因为 s 的所有权已经被转移

    let x = 5;
    makes_copy(x); // x 是一个基本数据类型，实现了 Copy trait，所以不会转移所有权
    println!("{}", x); // 这行代码可以正常运行
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

## 借用与引用

### 可变引用与不可变引用
借用是在不转移所有权的情况下访问数据的方式。Rust 支持不可变引用（使用 `&` 符号）和可变引用（使用 `&mut` 符号）。

示例代码：
```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // 不可变引用
    println!("The length of '{}' is {}.", s, len);

    let mut s2 = String::from("hello");
    change(&mut s2); // 可变引用
    println!("Changed string: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### 生命周期标注
当函数返回一个引用时，需要为引用指定生命周期标注，以确保引用在函数的整个生命周期内有效。

示例代码：
```rust
fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("short");
    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 示例代码：管理游戏状态

我们使用借用和引用管理推箱子游戏的游戏状态，以避免数据竞争和悬挂引用。

示例代码：
```rust
use std::io;

struct GameState {
    player_position: (i32, i32),
    box_position: (i32, i32),
    target_position: (i32, i32),
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_position: (0, 0),
            box_position: (1, 1),
            target_position: (2, 2),
        }
    }

    fn update(&mut self, direction: &str) {
        match direction {
            "up" => self.player_position.1 -= 1,
            "down" => self.player_position.1 += 1,
            "left" => self.player_position.0 -= 1,
            "right" => self.player_position.0 += 1,
            _ => (),
        }
    }
}

fn main() {
    let mut game_state = GameState::new();
    loop {
        println!("Player Position: {:?}", game_state.player_position);
        println!("Enter direction (up, down, left, right):");
        
        let mut direction = String::new();
        io::stdin().read_line(&mut direction).expect("Failed to read line");
        let direction = direction.trim();

        game_state.update(direction);

        if game_state.player_position == game_state.box_position {
            println!("You pushed the box!");
        }
    }
}
```

目前在游戏控制中，通过输入 `up` 等词汇来控制移动不太方便。可以考虑设置为使用 `WASD` 四个按键或者方向键来控制方向，以提供更便捷和直观的操作体验。

通过这部分的学习，您将掌握 Rust 中的所有权和借用机制，并能够使用借用和引用管理推箱子游戏的游戏状态。接下来，我们将介绍集合与枚举，并继续完善推箱子游戏的核心功能。
