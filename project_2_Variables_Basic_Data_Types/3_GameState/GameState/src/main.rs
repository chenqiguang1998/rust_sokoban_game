struct GameState {
    player_position: (i32, i32),
    box_position: (i32, i32),
    target_position: (i32, i32),
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_position: (0, 0),
            box_position: (1, 1),
            target_position: (2, 2),
        }
    }
}

fn main() {
    let game_state = GameState::new();
    println!(
        "Player Position: {:?}, Box Position: {:?}, Target Position: {:?}",
        game_state.player_position, game_state.box_position, game_state.target_position
    );
}

