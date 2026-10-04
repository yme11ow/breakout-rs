use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, ball::Ball, bricks::Bricks, player::Player};
use macroquad::{input::KeyCode::Escape, prelude::*};

pub struct GameState {
    pub lives: i32,
    pub score: i32,
    pub state: u8, // 0 is title, 1 is gameplay, 2 is game over, 3 is win, 4 is pause
    pub player: Player,
    pub bricks: Bricks,
    pub ball: Ball,
    pub show_debug: bool,
}

impl GameState {
    pub fn init() -> Self {
        let player = Player::spawn();
        let bricks = Bricks::spawn();
        let ball = Ball::spawn();

        GameState {
            lives: 5,
            score: 0,
            state: 1,
            player: player,
            bricks: bricks,
            ball: ball,
            show_debug: false,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.show_debug = !self.show_debug;
        }

        match self.state {
            0 => println!("Breakout: Rust Edition"),
            1 => {
                self.player.update();
                self.score += self.bricks.update(&mut self.ball);
                self.ball.update(&self.player.entity);
                self.lives();

                if self.score == 17600 {
                    self.state = 3;
                }

                if is_key_pressed(Escape) {
                    self.state = 4;
                }
            }
            2 | 3 => self.restart_game(),
            4 => {
                if is_key_pressed(Escape) {
                    self.state = 1;
                }
            }
            _ => (),
        }
    }

    pub fn draw(&mut self) {
        match self.state {
            0 => println!("Breakout Rust Edition"),
            1 => {
                clear_background(BLACK);
                set_camera(&Camera2D::from_display_rect(Rect::new(
                    0.0,
                    VIRTUAL_HEIGHT,
                    VIRTUAL_WIDTH,
                    -VIRTUAL_HEIGHT,
                )));
                self.player.draw();
                self.bricks.draw();
                self.ball.draw();
                draw_text(&format!("Score: {}", self.score), 10.0, 575.0, 20.0, WHITE);
                draw_text(&format!("Lives: {}", self.lives), 10.0, 590.0, 20.0, WHITE);
                set_default_camera();
                self.draw_debug();
            }
            2 => {
                set_camera(&Camera2D::from_display_rect(Rect::new(
                    0.0,
                    VIRTUAL_HEIGHT,
                    VIRTUAL_WIDTH,
                    -VIRTUAL_HEIGHT,
                )));
                let title_dims = measure_text("GAME OVER", None, 60, 1.0);
                let prompt_dims = measure_text("Press Enter to Restart", None, 30, 1.0);
                draw_text(
                    "GAME OVER",
                    VIRTUAL_WIDTH / 2.0 - title_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 2.0,
                    60.0,
                    RED,
                );
                draw_text(
                    "Press Enter to Restart",
                    VIRTUAL_WIDTH / 2.0 - prompt_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 2.0 + 30.0,
                    30.0,
                    RED,
                );
            }
            3 => {
                set_camera(&Camera2D::from_display_rect(Rect::new(
                    0.0,
                    VIRTUAL_HEIGHT,
                    VIRTUAL_WIDTH,
                    -VIRTUAL_HEIGHT,
                )));
                let title_dims = measure_text("YOU WIN", None, 60, 1.0);
                let prompt_dims = measure_text("Press Enter to Restart", None, 30, 1.0);
                draw_text(
                    "YOU WIN",
                    VIRTUAL_WIDTH / 2.0 - title_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 2.0,
                    60.0,
                    GREEN,
                );
                draw_text(
                    "Press Enter to Restart",
                    VIRTUAL_WIDTH / 2.0 - prompt_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 2.0 + 30.0,
                    30.0,
                    GREEN,
                );
            }
            4 => {
                self.player.draw_paused();
                self.bricks.draw_paused();
                self.ball.draw_paused();
                let title_dims = measure_text("PAUSED", None, 60, 1.0);
                let prompt_dims = measure_text("Press Esc to Continue", None, 30, 1.0);
                draw_text(
                    "PAUSED",
                    VIRTUAL_WIDTH / 2.0 - title_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 2.0,
                    60.0,
                    GRAY,
                );
                draw_text(
                    "Press Esc to Continue",
                    VIRTUAL_WIDTH / 2.0 - prompt_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 2.0 + 30.0,
                    30.0,
                    GRAY,
                );
            }
            _ => (),
        }
    }

    fn lives(&mut self) {
        if self.ball.missed_paddle() {
            self.lives -= 1;
        }

        if self.lives <= 0 {
            self.state = 2;
        }
    }

    fn draw_debug(&self) {
        if self.show_debug {
            draw_text(
                format!("FPS: {} dt: {:.2}ms", get_fps(), get_frame_time() * 1000.0),
                10.0,
                20.0,
                30.0,
                GREEN,
            );
        }
    }

    fn restart_game(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            let bricks = Bricks::spawn();
            let player = Player::spawn();
            self.state = 1;
            self.lives = 5;
            self.player = player;
            self.bricks = bricks;
            self.score = 0
        }
    }
}
