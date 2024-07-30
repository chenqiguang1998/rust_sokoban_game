// 此文件展示了如何使用枚举和模式匹配来处理游戏状态
use std::io;

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

impl Game {
    // 创建新的游戏实例
    fn new() -> Self {
        Game {
            state: GameState::Playing,
            player_position: (0, 0),
            box_position: (1, 1),
            target_position: (2, 2),
        }
    }

    // 根据输入方向更新游戏状态
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
    // 创建游戏实例
    let mut game = Game::new();
    loop {
        // 根据游戏状态进行不同的处理
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
