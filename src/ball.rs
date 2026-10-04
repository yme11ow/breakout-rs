use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, entity::Entity};
use ::rand;
use macroquad::color::{GRAY, WHITE};

#[derive(Default)]
pub struct BallHits {
    pub paddle: bool,
    pub wall: bool,
}

pub struct Ball {
    pub entity: Entity,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl Ball {
    pub fn spawn() -> Self {
        Ball {
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

    pub fn update(&mut self, player: &Entity) -> BallHits {
        let mut hits = BallHits::default();
        let random_bool: bool = rand::random();

        if self.missed_paddle() {
            self.entity.x = 400.0;
            self.entity.y = 300.0;

            if random_bool {
                self.velocity_x = -2.0;
            } else {
                self.velocity_x = 2.0;
            }
            self.velocity_y = 5.0;
        }

        hits.paddle = self.ball_to_player(player); //check collision with paddle

        // only count a wall hit when the velocity actually flips, so the sound
        // doesn't repeat while the ball is still overlapping the edge
        if self.entity.x < 0.0 && self.velocity_x < 0.0 {
            self.velocity_x = self.velocity_x.abs();
            hits.wall = true;
        } else if self.entity.x > VIRTUAL_WIDTH - self.entity.w && self.velocity_x > 0.0 {
            self.velocity_x = -self.velocity_x.abs();
            hits.wall = true;
        }

        if self.entity.y < 0.0 && self.velocity_y < 0.0 {
            self.velocity_y = self.velocity_y.abs();
            hits.wall = true;
        }

        self.entity.y += self.velocity_y;
        self.entity.x += self.velocity_x;

        hits
    }

    fn ball_to_player(&mut self, player: &Entity) -> bool {
        // only bounce while moving down, otherwise the ball can flip back and
        // forth (and replay the sound) while it overlaps the paddle
        if self.velocity_y > 0.0 && self.entity.intersects(player) {
            let ball_center = self.entity.x + self.entity.w / 2.0;
            let player_center = player.x + player.w / 2.0;
            let hit_pos = (ball_center - player_center) / (player.w / 2.0);
            let speed =
                (self.velocity_x * self.velocity_x + self.velocity_y * self.velocity_y).sqrt();

            self.velocity_x = hit_pos * speed;
            self.velocity_y = -self.velocity_y;
            true
        } else {
            false
        }
    }

    pub fn missed_paddle(&self) -> bool {
        if self.entity.y > VIRTUAL_HEIGHT - self.entity.h {
            true
        } else {
            false
        }
    }

    pub fn draw(&mut self) {
        self.entity.color = WHITE;
        self.entity.draw_circle(7.5);
    }

    pub fn draw_paused(&mut self) {
        self.entity.color = GRAY;
        self.entity.draw_circle(7.5);
    }
}
