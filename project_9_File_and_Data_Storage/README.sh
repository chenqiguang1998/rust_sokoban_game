
```markdown
# Rust 中的文件与数据存储

在这部分内容中，我们将介绍 Rust 中的文件读写操作以及如何实现推箱子游戏的进度保存和读取。

## 文件读写

### 读取文件
Rust 可以通过 `std::fs` 模块来读取文件的内容。

```rust
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("hello.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
```

### 写入文件
同样通过 `std::fs` 模块来写入文件的内容。

```rust
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
```


## 示例代码：保存和读取游戏进度

我们实现了保存和读取推箱子游戏进度的功能。

```rust
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};

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

    fn save(&self) -> std::io::Result<()> {
        let file = File::create("save.txt")?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{} {} {}", self.player_position.0, self.player_position.1, self.box_position.0, self.box_position.1)?;
        Ok(())
    }

    fn load(&mut self) -> std::io::Result<()> {
        let file = File::open("save.txt")?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 {
                self.player_position.0 = parts[0].parse().unwrap();
                self.player_position.1 = parts[1].parse().unwrap();
                self.box_position.0 = parts[2].parse().unwrap();
                self.box_position.1 = parts[3].parse().unwrap();
            }
        }
        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    game.load().unwrap();

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
                game.save().unwrap();
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


通过这部分的学习，您将掌握 Rust 中的文件与数据存储，并能够实现推箱子游戏的进度保存和读取功能。接下来，我们将介绍测试与调试，并继续完善推箱子游戏的功能。
```

