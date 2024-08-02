// 引入所需的模块
use std::sync::mpsc;  // 用于实现线程间的消息传递
use std::thread;  // 用于创建线程
use std::io;  // 用于输入输出操作

// 定义一个表示游戏状态的枚举
enum GameState {
    Playing,
    Won,
    Lost,
}

// 定义一个表示游戏的结构体
struct Game {
    state: GameState,  // 游戏的当前状态
    player_position: (i32, i32),  // 玩家的位置
    box_position: (i32, i32),  // 箱子的位置
    target_position: (i32, i32),  // 目标的位置
}

// 为 Game 结构体实现一个创建新游戏实例的方法
impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Playing,  // 初始状态为正在玩
            player_position: (0, 0),  // 玩家初始位置
            box_position: (1, 1),  // 箱子初始位置
            target_position: (2, 2),  // 目标初始位置
        }
    }

    // 根据输入的方向更新游戏状态的方法
    fn update(&mut self, direction: &str) {
        match direction {
            "up" => self.player_position.1 -= 1,
            "down" => self.player_position.1 += 1,
            "left" => self.player_position.0 -= 1,
            "right" => self.player_position.0 += 1,
            // 如果输入的方向无效，打印错误提示
            _ => println!("Invalid direction!"),
        }

        // 如果玩家位置与箱子位置相同，将游戏状态设置为赢
        if self.player_position == self.box_position {
            self.state = GameState::Won;
        }
    }
}

// 主函数
fn main() {
    // 创建一个新的游戏实例
    let mut game = Game::new();
    // 创建一个消息发送者（tx）和消息接收者（rx）
    let (tx, rx) = mpsc::channel();

    // 使用 thread::spawn 创建一个新的线程
    thread::spawn(move || {
        // 在线程中无限循环
        loop {
            // 创建一个可变的字符串用于存储用户输入
            let mut direction = String::new();
            // 从标准输入读取用户输入，并处理可能的错误
            io::stdin().read_line(&mut direction).expect("Failed to read line");
            // 去除输入字符串的前后空白，并转换为字符串
            let direction = direction.trim().to_string();
            // 向消息发送者发送用户输入的方向
            tx.send(direction).unwrap();
        }
    });

    // 无限循环，根据游戏状态进行处理
    loop {
        // 根据游戏的当前状态进行不同的处理
        match game.state {
            // 如果游戏正在进行
            GameState::Playing => {
                // 从消息接收者接收用户输入的方向
                let direction = rx.recv().unwrap();
                // 根据接收到的方向更新游戏状态
                game.update(&direction);
                // 打印玩家的当前位置
                println!("Player Position: {:?}", game.player_position);
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
