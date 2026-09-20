use macroquad::{prelude::*};
use macroquad::miniquad::conf::Platform;

// Structs & Important Variables
struct Entity {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
}

const VIRTUAL_WIDTH: f32 = 800.0;
const VIRTUAL_HEIGHT: f32 = 600.0;

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
    let mut player = Entity{x: VIRTUAL_WIDTH / 2.0 - 60.0, y: VIRTUAL_HEIGHT / 2.0 - 250.0, w: 100.0, h: 10.0, color: WHITE};

    loop {
        clear_background(BLACK);
        set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT)));
        game(&player);
        player_movement(&mut player);
        set_default_camera();
        draw_text(&format!("FPS: {} dt: {:.2}ms", get_fps(), get_frame_time() * 1000.0), 10.0, 20.0, 30.0, GREEN);
        next_frame().await
    }
}

fn game(player: &Entity) {
    draw_rectangle(player.x, player.y, player.w, player.h, player.color);
}

fn player_movement(player: &mut Entity) {
    let player_speed = 400.0;
    let dt = get_frame_time();

    if is_key_down(KeyCode::Right) {
        player.x += player_speed * dt;
    }
    if is_key_down(KeyCode::Left) {
        player.x -= player_speed * dt;
    }
}
