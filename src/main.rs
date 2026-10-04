mod audio;
mod ball;
mod bricks;
mod entity;
mod game_state;
mod player;
use crate::audio::Audio;
use crate::game_state::GameState;
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;

// Important Variables
pub const VIRTUAL_WIDTH: f32 = 800.0;
pub const VIRTUAL_HEIGHT: f32 = 600.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Breakout: Rust Edition".to_owned(),
        platform: Platform {
            swap_interval: Some(1), // 1 = vsync on, 0 = vsync off (note vsync off is buggy due to ball physics not using dt)
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    window().await;
}

async fn window() {
    let audio = Audio::load().await;
    let mut game_state = GameState::init(audio);

    loop {
        // update
        game_state.update();

        //draw
        game_state.draw();
        next_frame().await
    }
}
