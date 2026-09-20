use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;

// Structs & Important Variables
#[derive(Clone, Copy)]
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
    let mut player = Entity {
        x: VIRTUAL_WIDTH / 2.0 - 60.0,
        y: VIRTUAL_HEIGHT - 40.0,
        w: 100.0,
        h: 10.0,
        color: WHITE,
    };

    let brick = Entity {
        x: 0.0,
        y: 0.0,
        w: 25.0,
        h: 15.0,
        color: WHITE,
    };

    const COLS: usize = 32;
    const ROWS: usize = 10;

    let mut bricks: Vec<Entity> = vec![brick; ROWS * COLS];

    for y in 0..ROWS {
        for x in 0..COLS {
            bricks[x + y * COLS].x += x as f32 * 25.0;
            bricks[x + y * COLS].y += y as f32 * 15.0;
        }
    }

    loop {
        clear_background(BLACK);
        set_camera(&Camera2D::from_display_rect(Rect::new(
            0.0,
            VIRTUAL_HEIGHT,
            VIRTUAL_WIDTH,
            -VIRTUAL_HEIGHT,
        )));
        draw(&player);
        draw_bricks(&bricks, &brick);
        player_movement(&mut player);
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

fn draw(entity: &Entity) {
    draw_rectangle(entity.x, entity.y, entity.w, entity.h, entity.color);
}

fn draw_bricks(bricks: &[Entity], _brick: &Entity) {
    for brick in bricks {
        draw_rectangle(brick.x, brick.y, brick.w, brick.h, brick.color);
    }
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
