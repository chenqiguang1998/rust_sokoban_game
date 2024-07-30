
```markdown
# Rust 中的错误处理与结果类型

在这部分内容中，我们将介绍 Rust 中的错误处理机制，包括 `Result` 类型和 `Option` 类型，并通过示例代码处理推箱子游戏中的潜在错误。

## 错误处理

### `Result` 类型
`Result` 类型用于表示操作的结果，可能是成功（`Ok`）或失败（`Err`）。

```rust
fn main() {
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

### `Option` 类型
`Option` 类型用于表示一个值可能存在或不存在，可以是 `Some` 或 `None` 。

```rust
fn main() {
    let name = Some("Alice");
    match name {
        Some(n) => println!("Name: {}", n),
        None => println!("No name"),
    }

    let absent_name: Option<&str> = None;
    if let Some(n) = absent_name {
        println!("Name: {}", n);
    } else {
        println!("No name");
    }
}
```

## 示例代码：处理用户输入错误

我们处理用户输入错误，确保用户输入有效的方向命令。

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
            _ => println!("Invalid direction!"),
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

通过这部分的学习，您将掌握 Rust 中的错误处理和结果类型，并能够处理推箱子游戏中的潜在错误。接下来，我们将介绍模块和包，并继续完善推箱子游戏的功能。
```

