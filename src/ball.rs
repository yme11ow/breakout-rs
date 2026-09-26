use macroquad::color::WHITE;
use rand::{self};
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, entity::Entity};

pub struct Ball {
    pub entity: Entity,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl Ball{
    pub fn spawn() -> Self {
        Ball{
            entity: Entity {
                x: 400.0,
                y: 300.0,
                w: 15.0,
                h: 15.0,
                color: WHITE,  
            },
            velocity_x: 5.0,
            velocity_y: 5.0,
        }
    }

    pub fn update(&mut self, player: &Entity) {
        let random_bool: bool = rand::random();

        if self.entity.y > VIRTUAL_HEIGHT - self.entity.h {
            self.entity.x = 400.0;
            self.entity.y = 300.0;

            if random_bool {
                self.velocity_x = -5.0;
            } else {
                self.velocity_x = 5.0;
            }
            self.velocity_y = 5.0;
        }

        self.ball_to_player(player);

        if self.entity.x < 0.0 {
            self.velocity_x = self.velocity_x.abs();
        } else if self.entity.x > VIRTUAL_WIDTH - self.entity.w {
            self.velocity_x = -self.velocity_x.abs();
        }
        
        if self.entity.y < 0.0 {
            self.velocity_y = -self.velocity_y;
        }

        self.entity.y += self.velocity_y;
        self.entity.x += self.velocity_x;

    }

    fn ball_to_player(&mut self, player: &Entity) {
        if self.entity.intersects(player) {
            let ball_center = self.entity.x + self.entity.w / 2.0;
            let player_center = player.x + player.w / 2.0;
            let hit_pos = (ball_center - player_center) / (player.w / 2.0);
            let speed = (self.velocity_x * self.velocity_x + self.velocity_y * self.velocity_y).sqrt();

            self.velocity_x = hit_pos * speed;
            self.velocity_y = -self.velocity_y;
        }
    }

    pub fn draw(&self) {
        self.entity.draw_circle(7.5);
    }
}