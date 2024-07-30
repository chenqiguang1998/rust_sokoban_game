use std::io;

// 定义一个表示游戏状态的枚举类型
enum GameState {
    Playing,
    Won,
    Lost,
}

// 定义游戏结构体，包含游戏的状态和相关位置信息
struct Game {
    state: GameState,
    player_position: (i32, i32),
    box_position: (i32, i32),
    target_position: (i32, i32),
}

// 为 Game 结构体实现创建新游戏实例的方法
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

    // 根据输入的方向字符串更新游戏状态
    fn update(&mut self, direction: &str) {
        // 根据输入的方向执行相应的位置更新操作
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
}

// 主函数
fn main() {
    // 创建一个新的游戏实例
    let mut game = Game::new();
    // 进入一个无限循环，直到游戏结束
    loop {
        // 根据游戏状态进行不同的处理
        match game.state {
            // 如果游戏状态是 Playing
            GameState::Playing => {
                // 打印玩家当前位置
                println!("Player Position: {:?}", game.player_position);
                // 提示用户输入方向
                println!("Enter direction (up, down, left, right):");

                // 创建一个可变的字符串用于存储用户输入
                let mut direction = String::new();
                // 从标准输入读取用户输入，并处理可能的错误
                io::stdin().read_line(&mut direction).expect("Failed to read line");
                // 去除输入字符串的前后空白
                let direction = direction.trim();

                // 根据用户输入的方向更新游戏状态
                game.update(direction);
            }
            // 如果游戏状态是 Won，打印胜利消息并退出循环
            GameState::Won => {
                println!("You won the game!");
                break;
            }
            // 如果游戏状态是 Lost，打印失败消息并退出循环
            GameState::Lost => {
                println!("You lost the game!");
                break;
            }
        }
    }
}
