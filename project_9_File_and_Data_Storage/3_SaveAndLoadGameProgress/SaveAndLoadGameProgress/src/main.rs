// 此文件展示了如何在 Rust 中实现推箱子游戏的进度保存和读取功能
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::sync::mpsc;
use std::thread;

// 定义游戏状态的枚举
enum GameState {
    Playing,
    Won,
    Lost,
}

// 定义游戏结构体
struct Game {
    state: GameState,
    player_position: (i32, i32),
    box_position: (i32, i32),
    target_position: (i32, i32),
}

// 为游戏结构体实现创建新游戏实例的方法
impl Game {
    // 创建并返回一个新的游戏实例
    fn new() -> Self {
        Game {
            state: GameState::Playing,
            player_position: (0, 0),
            box_position: (1, 1),
            target_position: (2, 2),
        }
    }

    // 根据输入的方向更新游戏状态
    fn update(&mut self, direction: &str) {
        match direction {
            "up" => self.player_position.1 -= 1,
            "down" => self.player_position.1 += 1,
            "left" => self.player_position.0 -= 1,
            "right" => self.player_position.0 += 1,
            // 如果输入的方向无效，打印错误提示
            _ => println!("Invalid direction!"),
        }

        // 如果玩家位置与箱子位置相同，将游戏状态设置为 Won
        if self.player_position == self.box_position {
            self.state = GameState::Won;
        }
    }

    // 保存游戏进度的方法
    fn save(&self) -> std::io::Result<()> {
        // 创建或覆盖名为 "save.txt" 的文件
        let file = File::create("save.txt")?;
        // 创建一个基于该文件的缓冲区写入器
        let mut writer = BufWriter::new(file);
        // 将玩家和箱子的位置信息写入文件
        writeln!(writer, "{} {} {} {}", self.player_position.0, self.player_position.1, self.box_position.0, self.box_position.1)?;
        Ok(())
    }

    // 加载游戏进度的方法
    fn load(&mut self) -> std::io::Result<()> {
        // 打开名为 "save.txt" 的文件
        let file = File::open("save.txt")?;
        // 创建一个基于该文件的缓冲区读取器
        let reader = BufReader::new(file);
        // 遍历读取器中的每一行
        for line in reader.lines() {
            let line = line?;
            // 按空格分割行内容为部分
            let parts: Vec<&str> = line.split_whitespace().collect();
            // 如果分割后的部分数量为 4
            if parts.len() == 4 {
                // 解析并更新玩家和箱子的位置
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
    // 创建一个新的游戏实例
    let mut game = Game::new();
    // 加载游戏进度
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
        // 根据游戏的当前状态进行不同的处理
        match game.state {
            // 如果游戏正在进行
            GameState::Playing => {
                // 从通道接收方向输入
                let direction = rx.recv().unwrap();
                // 根据方向更新游戏状态
                game.update(&direction);
                // 打印玩家的当前位置
                println!("Player Position: {:?}", game.player_position);
                // 保存游戏进度
                game.save().unwrap();
            }
            // 如果游戏胜利
            GameState::Won => {
                // 打印胜利消息，并退出循环
                println!("You won the game!");
                break;
            }
            // 如果游戏失败
            GameState::Lost => {
                // 打印失败消息，并退出循环
                println!("You lost the game!");
                break;
            }
        }
    }
}
