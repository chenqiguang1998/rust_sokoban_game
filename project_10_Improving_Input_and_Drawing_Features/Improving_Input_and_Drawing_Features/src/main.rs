use ggez::graphics::Canvas;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use winit::event::VirtualKeyCode as KeyCode; // 使用 VirtualKeyCode 代替 KeyCode
use ggez::input::keyboard::KeyMods; // 直接使用 KeyMods
use ggez::audio::{SoundSource, Source};
use std::time::Instant;

struct Game {
    player_position: (f32, f32),
    box_position: (f32, f32),
    target_position: (f32, f32),
    obstacles: Vec<Rect>,
    control_scheme: ControlScheme,
    steps: u32,
    start_time: Instant,
    background_music: Option<Source>,
    push_sound: Option<Source>,
    obstacle_collision_sound: Option<Source>,
    box_push_sound: Option<Source>,
}

enum ControlScheme {
    WASD,
    ArrowKeys,
}

impl Game {
    fn new(ctx: &mut Context) -> Self {
        let background_music = Source::new(ctx, "/sounds/background_music.mp3").ok();
        let push_sound = Source::new(ctx, "/sounds/push.wav").ok();
        let obstacle_collision_sound = Source::new(ctx, "/sounds/collision.wav").ok();
        let box_push_sound = Source::new(ctx, "/sounds/box_push.wav").ok();

        Game {
            player_position: (100.0, 100.0),
            box_position: (200.0, 200.0),
            target_position: (300.0, 300.0),
            obstacles: vec![],
            control_scheme: ControlScheme::WASD,
            steps: 0,
            start_time: Instant::now(),
            background_music,
            push_sound,
            obstacle_collision_sound,
            box_push_sound,
        }
    }

    fn play_sound(&mut self, sound: &mut ggez::audio::Source) {
        sound.play();
    }

    fn update_position(&mut self, direction: KeyCode) {
        let old_player_position = self.player_position;
        let old_box_position = self.box_position;

        match self.control_scheme {
            ControlScheme::WASD => {
                match direction {
                    KeyCode::W => self.player_position.1 -= 10.0,
                    KeyCode::S => self.player_position.1 += 10.0,
                    KeyCode::A => self.player_position.0 -= 10.0,
                    KeyCode::D => self.player_position.0 += 10.0,
                    _ => (),
                }
            }
            ControlScheme::ArrowKeys => {
                match direction {
                    KeyCode::Up => self.player_position.1 -= 10.0,
                    KeyCode::Down => self.player_position.1 += 10.0,
                    KeyCode::Left => self.player_position.0 -= 10.0,
                    KeyCode::Right => self.player_position.0 += 10.0,
                    _ => (),
                }
            }
        }

        // 检测是否碰撞到障碍物
        if self.obstacles.iter().any(|obstacle| obstacle.contains([self.player_position.0, self.player_position.1])) {
            self.player_position = old_player_position;
            if let Some(ref mut sound) = self.obstacle_collision_sound {
                play_sound(sound).ok();
            }
            return;
        }

        // 检测是否推到箱子
        if (self.player_position.0 - self.box_position.0).abs() < 10.0
            && (self.player_position.1 - self.box_position.1).abs() < 10.0
        {
            match direction {
                KeyCode::W => self.box_position.1 -= 10.0,
                KeyCode::S => self.box_position.1 += 10.0,
                KeyCode::A => self.box_position.0 -= 10.0,
                KeyCode::D => self.box_position.0 += 10.0,
                _ => (),
            }
            if let Some(ref mut sound) = self.box_push_sound {
                play_sound(sound).ok();
            }
        }

        // 播放推动箱子的音效
        if self.player_position != old_player_position {
            if let Some(ref mut sound) = self.push_sound {
                play_sound(sound).ok();
            }
        }

        self.steps += 1;

        if self.player_position == self.target_position && self.box_position == self.target_position {
            // 完成游戏逻辑
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        Canvas.clear(ctx, Color::WHITE);

        // 绘制玩家
        let player_rect = Rect::new(self.player_position.0, self.player_position.1, 20.0, 20.0);
        let player_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), player_rect, Color::BLUE)?;

        // 绘制箱子
        let box_rect = Rect::new(self.box_position.0, self.box_position.1, 20.0, 20.0);
        let box_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), box_rect, Color::RED)?;

        // 绘制目标
        let target_rect = Rect::new(self.target_position.0, self.target_position.1, 20.0, 20.0);
        let target_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), target_rect, Color::GREEN)?;

        // 绘制障碍物
        let obstacle_meshes: Vec<_> = self.obstacles.iter()
            .map(|obstacle| Mesh::new_rectangle(ctx, DrawMode::fill(), *obstacle, Color::YELLOW))
            .collect::<Result<Vec<_>, _>>()?;

        graphics::draw(ctx, &player_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &box_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &target_mesh, graphics::DrawParam::default())?;

        for mesh in obstacle_meshes {
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        // 显示步数和时间
        let elapsed_time = self.start_time.elapsed().as_secs();
        let step_text = format!("Steps: {}", self.steps);
        let time_text = format!("Time: {}s", elapsed_time);

        let step_display = graphics::Text::new(step_text);
        let time_display = graphics::Text::new(time_text);

        graphics::draw(ctx, &step_display, (ggez::mint::Point2 { x: 10.0, y: 10.0 }, Color::BLACK))?;
        graphics::draw(ctx, &time_display, (ggez::mint::Point2 { x: 10.0, y: 30.0 }, Color::BLACK))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(keycode) = ctx.keyboard_input().keycode {
            self.update_position(keycode);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        self.update_position(keycode);
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("push_box_game", "Author")
        .add_resource_path("resources") // 确保你有正确的资源路径
        .build()?;

    let mut game = Game::new(&mut ctx);
    if let Some(ref mut music) = game.background_music {
        play_sound(music).ok(); // 播放背景音乐
    }
    ggez::event::run(ctx, event_loop, &mut game)
}
