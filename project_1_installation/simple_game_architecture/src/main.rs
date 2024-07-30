extern crate ggez;
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};

struct GameState;

impl GameState {
    fn new() -> Self {
        GameState
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("sokoban", "author");
    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
