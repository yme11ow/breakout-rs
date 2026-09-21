use crate::entity::Entity;
use macroquad::prelude::*;
// use crate::{VIRTUAL_WIDTH, VIRTUAL_HEIGHT};

fn init_brick() -> Entity {
    let brick = Entity {
        x: 0.0,
        y: 0.0,
        w: 25.0,
        h: 15.0,
        color: WHITE,
    };

    brick
}

pub fn init_grid() -> Vec<Entity> {
    let brick = init_brick();

    const COLS: usize = 32;
    const ROWS: usize = 10;
    const ROW_COLORS: [Color; ROWS] = [
        RED, ORANGE, YELLOW, GOLD, GREEN, BLUE, PURPLE, MAGENTA, PINK, GRAY,
    ];

    let mut bricks: Vec<Entity> = vec![brick; ROWS * COLS];

    for y in 0..ROWS {
        for x in 0..COLS {
            bricks[x + y * COLS].x += x as f32 * 25.0;
            bricks[x + y * COLS].y += y as f32 * 15.0;
            bricks[x + y * COLS].color = ROW_COLORS[y];
        }
    }

    bricks
}

pub fn make_bricks(bricks: &Vec<Entity>) {
    for brick in bricks {
        brick.draw();
    }
}