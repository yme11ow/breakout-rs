use crate::entity::Entity;
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use macroquad::prelude::*;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn spawn() -> Self {
        Player {
            entity: Entity {
                x: VIRTUAL_WIDTH / 2.0 - 60.0,
                y: VIRTUAL_HEIGHT - 40.0,
                w: 100.0,
                h: 10.0,
                color: WHITE,
            },
        }
    }

    pub fn update(&mut self) {
        let player_speed = 400.0;
        let dt = get_frame_time();

        if is_key_down(KeyCode::Right) {
            self.entity.x += player_speed * dt;
        }
        if is_key_down(KeyCode::Left) {
            self.entity.x -= player_speed * dt;
        }

        if self.entity.x > VIRTUAL_WIDTH {
            self.entity.x += -VIRTUAL_WIDTH - self.entity.w;
        }
        if self.entity.x + self.entity.w <= 0.0 {
            self.entity.x += VIRTUAL_WIDTH + self.entity.w;
        }
    }

    pub fn draw(&self) {
        self.entity.draw();
    }
}
