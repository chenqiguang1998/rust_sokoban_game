

```markdown
# Rust 中的多线程与并发

在这部分内容中，我们将深入探讨 Rust 中的多线程与并发概念，并通过示例代码实现推箱子游戏的并发处理。

## 多线程

### 创建线程
Rust 可以通过 `std::thread` 模块来创建和管理线程。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello from the main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```


## 并发与消息传递

### 消息传递
Rust 能够通过 `std::sync::mpsc` 模块实现线程间的消息传递。

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```


## 示例代码：并发处理游戏输入

我们使用多线程与消息传递来实现对用户输入的并发处理。

```rust
use std::sync::mpsc;
use std::thread;
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
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut direction = String::new();
            io::stdin().read_line(&mut direction).expect("Failed to read line");
            let direction = direction.trim().to_string();
            tx.send(direction).unwrap();
        }
    });

    loop {
        match game.state {
            GameState::Playing => {
                let direction = rx.recv().unwrap();
                game.update(&direction);
                println!("Player Position: {:?}", game.player_position);
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


通过这部分的学习，您将掌握 Rust 中的多线程与并发，并能够运用多线程和消息传递来实现推箱子游戏的并发处理。接下来，我们将介绍文件与数据存储，并继续完善推箱子游戏的功能。
```

