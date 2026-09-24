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
}


fn movement(ball: &mut Ball, player: & mut Entity) {
    let random_bool: bool = rand::random();

    if ball.entity.y > VIRTUAL_HEIGHT - ball.entity.h {
        ball.entity.x = 400.0;
        ball.entity.y = 300.0;

        if random_bool {
            ball.velocity_x = -5.0;
        } else {
            ball.velocity_x = 5.0;
        }
        ball.velocity_y = 5.0;
        // lives - 1?
    }

    if ball.entity.x < 0.0 {
        ball.velocity_x = ball.velocity_x.abs();
    } else if ball.entity.x > VIRTUAL_WIDTH - ball.entity.w {
        ball.velocity_x = -ball.velocity_x.abs();
    }
    
    if ball.entity.y < 0.0 {
        ball.velocity_y = -ball.velocity_y;
    }

    if ball.entity.intersects(player) {
        let ball_center = ball.entity.x + ball.entity.w / 2.0;
        let player_center = player.x + player.w / 2.0;
        let hit_pos = (ball_center - player_center) / (player.w / 2.0);
        let speed = (ball.velocity_x * ball.velocity_x + ball.velocity_y * ball.velocity_y).sqrt();

        ball.velocity_x = hit_pos * speed;
        ball.velocity_y = -ball.velocity_y;
    }

    ball.entity.y += ball.velocity_y;
    ball.entity.x += ball.velocity_x;

}

pub fn make_ball(ball: &mut Ball, player:&mut Entity) {
    ball.entity.draw_circle(7.5);
    movement(ball, player);
}