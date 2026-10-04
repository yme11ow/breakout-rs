use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, audio::Audio, ball::Ball, bricks::Bricks, player::Player};
use macroquad::{input::KeyCode::Escape, prelude::*};

pub struct GameState {
    pub lives: i32,
    pub score: i32,
    pub state: u8, // 0 is title, 1 is gameplay, 2 is game over, 3 is win, 4 is pause
    pub player: Player,
    pub bricks: Bricks,
    pub ball: Ball,
    pub show_debug: bool,
    pub audio: Audio,
}

impl GameState {
    pub fn init(audio: Audio) -> Self {
        audio.play_title_music();

        let player = Player::spawn();
        let bricks = Bricks::spawn();
        let ball = Ball::spawn();

        GameState {
            lives: 5,
            score: 0,
            state: 0,
            player: player,
            bricks: bricks,
            ball: ball,
            show_debug: false,
            audio: audio,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.show_debug = !self.show_debug;
        }

        match self.state {
            1 => {
                self.player.update();
                let points = self.bricks.update(&mut self.ball);
                if points > 0 {
                    self.audio.ball_brick();
                }
                self.score += points;

                let hits = self.ball.update(&self.player.entity);
                if hits.paddle {
                    self.audio.ball_paddle();
                }
                if hits.wall {
                    self.audio.ball_wall();
                }

                self.lives();

                if self.score == 17600 {
                    self.state = 3;
                    self.audio.play_victory_music();
                }

                if is_key_pressed(Escape) {
                    self.state = 4;
                }
            }
            0 | 2 | 3 => self.start_game(),
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
            0 => {
                set_camera(&Camera2D::from_display_rect(Rect::new(
                    0.0,
                    VIRTUAL_HEIGHT,
                    VIRTUAL_WIDTH,
                    -VIRTUAL_HEIGHT,
                )));
                let title_dims = measure_text("BREAKOUT", None, 60, 1.0);
                let subtitle_dims = measure_text("Rust Edition", None, 30, 1.0);
                let prompt_dims = measure_text("Press Enter to Start", None, 30, 1.0);
                draw_text(
                    "BREAKOUT",
                    VIRTUAL_WIDTH / 2.0 - title_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 4.0,
                    60.0,
                    ORANGE,
                );
                draw_text(
                    "Rust Edition",
                    VIRTUAL_WIDTH / 2.0 - subtitle_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 4.0 + 30.0,
                    30.0,
                    ORANGE,
                );
                draw_text(
                    "Press Enter to Start",
                    VIRTUAL_WIDTH / 2.0 - prompt_dims.width / 2.0,
                    VIRTUAL_HEIGHT / 2.0 + 60.0,
                    30.0,
                    ORANGE,
                );
            }
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
                draw_text(&format!("Score: {}", self.score), self.player.entity.x, self.player.entity.y + 22.0, 20.0, WHITE); // OG x: 10, y: 575
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
            self.audio.play_game_over_music();
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

    fn start_game(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            if self.state == 0 {
                self.audio.start_game();
            } else {
                self.audio.click_button();
            }
            self.audio.play_gameplay_music();

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
