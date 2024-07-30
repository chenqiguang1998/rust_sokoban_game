
# Rust 中的函数与控制流

在这部分内容中，我们将深入探讨 Rust 中的函数定义与调用以及控制流语句，并通过示例代码展示如何实现推箱子游戏的游戏循环。

## 函数定义与调用

### 函数定义
在 Rust 中，使用 `fn` 关键字定义函数，函数名后跟随参数列表和返回值类型。

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 函数参数与返回值
函数参数在参数列表中定义，多个参数用逗号分隔，可指定返回值类型。

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### 函数的生命周期标注
在函数中使用引用时，可能需要指定生命周期标注以确保引用的有效性。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 控制流

### if 条件语句
用于根据条件的真假执行不同的代码块。

```rust
fn main() {
    let number = 5;

    if number % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }
}
```

### 循环语句
Rust 支持多种循环语句，如 `for`、`while` 和 `loop` 。

- `for` 循环：

```rust
fn main() {
    for i in 1..5 {
        println!("i: {}", i);
    }
}
```

- `while` 循环：

```rust
fn main() {
    let mut number = 3;
    while number!= 0 {
        println!("number: {}", number);
        number -= 1;
    }
}
```

- `loop` 循环：

```rust
fn main() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!("Count reached: {}", count);
}
```

### match 模式匹配
根据值匹配不同的分支。

```rust
fn main() {
    let number = 5;

    match number {
        1 => println!("One"),
        2 | 3 | 5 => println!("Prime"),
        4..=10 => println!("Between 4 and 10"),
        _ => println!("Something else"),
    }
}
```

## 示例代码：游戏循环

我们实现了一个简单的游戏循环，根据用户输入更新游戏状态。

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

通过这部分的学习，您将掌握 Rust 中的函数与控制流，并能够实现推箱子游戏的基本游戏循环。接下来，我们将介绍所有权与借用，并继续完善推箱子游戏的核心功能。

