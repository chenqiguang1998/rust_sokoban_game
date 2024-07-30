# Rust 中的集合与枚举

在这部分内容中，我们将介绍 Rust 中的集合类型（如 Vector、HashMap、String）和枚举，并通过示例代码进一步丰富推箱子游戏的功能。

## 集合类型

### Vector
Vector 是一种动态数组，可以存储任意数量的相同类型的元素。

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let v2 = vec![1, 2, 3];
    for i in &v2 {
        println!("{}", i);
    }
}
```

### HashMap
HashMap 是一种键值对集合，键和值可以是任意类型。

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(&score) => println!("Score: {}", score),
        None => println!("No score found."),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

### String
String 是一种动态可变的字符串类型，可以存储任意数量的字符。

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
```

## 枚举与模式匹配

### 定义枚举
枚举是一种定义一组相关值的数据类型，每个枚举值可以有不同的类型和数量的数据。

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}
```

### 使用枚举和模式匹配处理游戏状态
我们使用枚举表示游戏状态，并使用模式匹配处理游戏的不同状态。

```rust
use std::io;

enum GameState {
    Playing,
    Won,
    Lost,
}

struct Game {
    state: GameState,
    player_position: (i32, i32),
    box_position: (i32, i32),
    target_position: (i32, i32),
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Playing,
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

        if self.player_position == self.box_position {
            self.state = GameState::Won;
        }
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        match game.state {
            GameState::Playing => {
                println!("Player Position: {:?}", game.player_position);
                println!("Enter direction (up, down, left, right):");

                let mut direction = String::new();
                io::stdin().read_line(&mut direction).expect("Failed to read line");
                let direction = direction.trim();

                game.update(direction);
            }
            GameState::Won => {
                println!("You won the game!");
                break;
            }
            GameState::Lost => {
                println!("You lost the game!");
                break;
            }
        }
    }
}
```

通过这部分的学习，您将掌握 Rust 中的集合类型和枚举，并能够使用枚举和模式匹配处理推箱子游戏的不同状态。接下来，我们将介绍错误处理和结果类型，并继续完善推箱子游戏的功能。
