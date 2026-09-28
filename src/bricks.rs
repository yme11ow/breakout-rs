use crate::{ball::Ball, entity::Entity};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Brick {
    pub entity: Entity,
}

impl Brick {
    fn spawn() -> Self {
        Brick {
            entity: Entity {
                x: 0.0,
                y: 0.0,
                w: 25.0,
                h: 15.0,
                color: WHITE,
            },
        }
    }

    fn draw(&self) {
        self.entity.draw();
    }
}

pub struct Bricks {
    list: Vec<Brick>,
}

impl Bricks {
    pub fn spawn() -> Self {
        let brick = Brick::spawn();

        const COLS: usize = 32;
        const ROWS: usize = 10;
        const ROW_COLORS: [Color; ROWS] = [
            RED, ORANGE, YELLOW, GOLD, GREEN, BLUE, PURPLE, MAGENTA, PINK, GRAY,
        ];

        let mut list: Vec<Brick> = vec![brick; ROWS * COLS];

        for y in 0..ROWS {
            for x in 0..COLS {
                let brick = &mut list[x + y * COLS].entity;
                brick.x += x as f32 * 25.0;
                brick.y += y as f32 * 15.0;
                brick.color = ROW_COLORS[y];
            }
        }

        Self { list }
    }

    pub fn update(&mut self, ball: &mut Ball) {
        self.list.retain(|brick| {
            if ball.entity.intersects(&brick.entity) {
                ball.velocity_y = -ball.velocity_y;
                false
            } else {
                true
            }
        });
    }

    pub fn draw(&self) {
        for brick in self.list.iter() {
            brick.draw();
        }
    }
}
