use std::io;

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

    fn update(&mut self, direction: &str) {
        match direction {
            "up" => self.player_position.1 -= 1,
            "down" => self.player_position.1 += 1,
            "left" => self.player_position.0 -= 1,
            "right" => self.player_position.0 += 1,
            _ => (),
        }
    }
}

fn main() {
    let mut game_state = GameState::new();
    loop {
        println!("Player Position: {:?}", game_state.player_position);
        println!("Enter direction (up, down, left, right) or 'q' to quit:");

        let mut direction = String::new();
        io::stdin().read_line(&mut direction).expect("Failed to read line");
        let direction = direction.trim();

        if direction == "q" {
            break;
        }

        game_state.update(direction);

        if game_state.player_position == game_state.box_position {
            println!("You pushed the box!");
        }
    }
}

