
```markdown
# Rust 中的模块与包

在这部分内容中，我们将深入介绍 Rust 中的模块和包的概念，并通过示例代码展示如何组织推箱子游戏的代码结构。

## 模块

### 定义模块
模块用于将相关的功能组织在一起，使代码更具结构化和可读性。

```rust
mod game {
    pub struct GameState {
        pub player_position: (i32, i32),
        pub box_position: (i32, i32),
        pub target_position: (i32, i32),
    }

    impl GameState {
        pub fn new() -> Self {
            GameState {
                player_position: (0, 0),
                box_position: (1, 1),
                target_position: (2, 2),
            }
        }

        pub fn update(&mut self, direction: &str) {
            match direction {
                "up" => self.player_position.1 -= 1,
                "down" => self.player_position.1 += 1,
                "left" => self.player_position.0 -= 1,
                "right" => self.player_position.0 += 1,
                _ => println!("Invalid direction!"),
            }

            if self.player_position == self.box_position {
                println!("You pushed the box!");
            }
        }
    }
}

fn main() {
    let mut game_state = game::GameState::new();
    println!("Player Position: {:?}", game_state.player_position);
}
```

### 子模块
子模块可以进一步细分模块的结构，使代码组织更加清晰。

```rust
mod game {
    pub mod state {
        pub struct GameState {
            pub player_position: (i32, i32),
            pub box_position: (i32, i32),
            pub target_position: (i32, i32),
        }

        impl GameState {
            pub fn new() -> Self {
                GameState {
                    player_position: (0, 0),
                    box_position: (1, 1),
                    target_position: (2, 2),
                }
            }

            pub fn update(&mut self, direction: &str) {
                match direction {
                    "up" => self.player_position.1 -= 1,
                    "down" => self.player_position.1 += 1,
                    "left" => self.player_position.0 -= 1,
                    "right" => self.player_position.0 += 1,
                    _ => println!("Invalid direction!"),
                }

                if self.player_position == self.box_position {
                    println!("You pushed the box!");
                }
            }
        }
    }
}

fn main() {
    let mut game_state = game::state::GameState::new();
    println!("Player Position: {:?}", game_state.player_position);
}
```

## 包与 Cargo

### 创建包
使用 `Cargo` 可以方便地创建 Rust 包，并管理项目的结构和依赖。

```bash
cargo new sokoban
cd sokoban
```

### `Cargo.toml` 文件
`Cargo.toml` 用于配置项目的基本信息，如名称、版本、依赖等。

```toml
[package]
name = "sokoban"
version = "0.1.0"
edition = "2018"

[dependencies]
```

### 主模块
在 `src/main.rs` 文件中编写项目的主模块代码。

```rust
fn main() {
    println!("Welcome to Sokoban!");
}
```

### 构建与运行
使用 `Cargo` 进行项目的构建和运行。

```bash
cargo build
cargo run
```

通过这部分的学习，您将掌握 Rust 中的模块和包，并能够有效地组织推箱子游戏的代码结构。接下来，我们将介绍多线程与并发，并继续完善推箱子游戏的功能。
```

