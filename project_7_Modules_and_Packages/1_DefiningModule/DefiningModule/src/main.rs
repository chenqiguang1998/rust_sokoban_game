// 定义一个名为 game 的模块
mod game {
    // 定义一个公共的结构体 GameState，表示游戏状态
    pub struct GameState {
        // 玩家的位置，用 (i32, i32) 表示
        pub player_position: (i32, i32),
        // 箱子的位置，用 (i32, i32) 表示
        pub box_position: (i32, i32),
        // 目标位置，用 (i32, i32) 表示
        pub target_position: (i32, i32),
    }

    // 为 GameState 结构体实现方法
    impl GameState {
        // 创建一个新的 GameState 实例的方法
        pub fn new() -> Self {
            // 初始化游戏状态的位置
            GameState {
                player_position: (0, 0),
                box_position: (1, 1),
                target_position: (2, 2),
            }
        }

        // 根据输入的方向更新游戏状态的方法
        pub fn update(&mut self, direction: &str) {
            // 根据输入的方向执行相应的位置变更操作
            match direction {
                "up" => self.player_position.1 -= 1,
                "down" => self.player_position.1 += 1,
                "left" => self.player_position.0 -= 1,
                "right" => self.player_position.0 += 1,
                // 如果输入的方向无效，打印错误提示
                _ => println!("Invalid direction!"),
            }

            // 如果玩家位置与箱子位置相同，打印提示信息
            if self.player_position == self.box_position {
                println!("You pushed the box!");
            }
        }
    }
}

// 主函数
fn main() {
    // 创建一个 game 模块中的 GameState 结构体的可变实例
    let mut game_state = game::GameState::new();
    // 打印玩家的初始位置
    println!("Player Position: {:?}", game_state.player_position);
}
