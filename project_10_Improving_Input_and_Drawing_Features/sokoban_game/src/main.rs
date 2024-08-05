use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    player_position: (f32, f32),
    box_positions: Vec<(f32, f32)>,
    target_positions: Vec<(f32, f32)>,
    move_count: u32,
    level: u32,
    time_elapsed: f32,
}

impl GameState {
    fn new(player_position: (f32, f32), box_positions: Vec<(f32, f32)>, target_positions: Vec<(f32, f32)>, level: u32) -> Self {
        Self {
            player_position,
            box_positions,
            target_positions,
            move_count: 0,
            level,
            time_elapsed: 0.0,
        }
    }

    fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(filename)?;
        let game_state = serde_json::from_reader(file)?;
        Ok(game_state)
    }

    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(filename)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }
}

fn save_game(state: &GameState) {
    if let Err(e) = state.save_to_file("savegame.json") {
        println!("Failed to save game: {}", e);
    }
}

fn load_game() -> Option<GameState> {
    match GameState::load_from_file("savegame.json") {
        Ok(state) => Some(state),
        Err(e) => {
            println!("Failed to load game: {}", e);
            None
        }
    }
}

fn main() {
    // 初始化一个游戏状态
    let game_state = GameState::new(
        (1.0, 1.0), 
        vec![(2.0, 2.0), (3.0, 3.0)], 
        vec![(4.0, 4.0)], 
        1
    );

    // 保存游戏状态到文件
    save_game(&game_state);

    // 从文件加载游戏状态
    if let Some(loaded_state) = load_game() {
        println!("Loaded game state: {:?}", loaded_state);
    } else {
        println!("Failed to load game state.");
    }
}
