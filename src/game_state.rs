use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, ball::Ball, bricks::Bricks, player::Player};
use macroquad::prelude::*;

pub struct GameState {
    pub lives: i32,
    pub score: i32,
    pub state: u8, // 0 is title, 1 is gameplay, 2 is game over, 3 is pause
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

        GameState{
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
                self.bricks.update(&mut self.ball);
                self.ball.update(&self.player.entity);
                self.lives();
            },
            2 => {
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
            3 => println!("Pause"),
            _ => println!("Nothing"),
        }
    }

    pub fn draw(&self) {
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
                draw_text(&format!("Lives: {}", self.lives), 10.0, 580.0, 20.0, WHITE);
                set_default_camera();
                self.draw_debug();
            },
            2 => {
                set_camera(&Camera2D::from_display_rect(Rect::new(
                    0.0,
                    VIRTUAL_HEIGHT,
                    VIRTUAL_WIDTH,
                    -VIRTUAL_HEIGHT,
                )));
                draw_text("GAME OVER", VIRTUAL_WIDTH / 2.0 - 120.0, VIRTUAL_HEIGHT / 2.0, 60.0, RED);
                draw_text("Press Enter to Restart", VIRTUAL_WIDTH / 2.0 - 144.0, VIRTUAL_HEIGHT / 2.0 + 30.0, 30.0, RED);
            },
            3 => println!("pause"),
            _ => println!("nothing")
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
}
