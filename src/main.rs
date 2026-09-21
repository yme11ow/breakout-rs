mod entity;
mod player;
mod bricks;
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;

// Important Variables
pub const VIRTUAL_WIDTH: f32 = 800.0;
pub const VIRTUAL_HEIGHT: f32 = 600.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Breakout: Rust Edition".to_owned(),
        platform: Platform {
            swap_interval: Some(0), // 1 = vsync on
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
    let mut player = player::spawn();
    let mut bricks = bricks::init_grid();

    loop {
        clear_background(BLACK);
        set_camera(&Camera2D::from_display_rect(Rect::new(
            0.0,
            VIRTUAL_HEIGHT,
            VIRTUAL_WIDTH,
            -VIRTUAL_HEIGHT,
        )));
        player::make_player(&mut player);
        bricks::make_bricks(&bricks);
        set_default_camera();
        draw_text(
            format!("FPS: {} dt: {:.2}ms", get_fps(), get_frame_time() * 1000.0),
            10.0,
            20.0,
            30.0,
            GREEN,
        );
        next_frame().await
    }
}

