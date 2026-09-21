use crate::entity::Entity;
use macroquad::prelude::*;
use crate::{VIRTUAL_WIDTH, VIRTUAL_HEIGHT};

pub fn spawn() -> Entity {
    Entity {
        x: VIRTUAL_WIDTH / 2.0 - 60.0,
        y: VIRTUAL_HEIGHT - 40.0,
        w: 100.0,
        h: 10.0,
        color: WHITE,
    }
}

fn movement(player: &mut Entity) {
    let player_speed = 400.0;
    let dt = get_frame_time();

    if is_key_down(KeyCode::Right) {
        player.x += player_speed * dt;
    }
    if is_key_down(KeyCode::Left) {
        player.x -= player_speed * dt;
    }

    if player.x > VIRTUAL_WIDTH {
        player.x += -VIRTUAL_WIDTH - player.w;
    }
    if player.x + player.w <= 0.0 {
        player.x += VIRTUAL_WIDTH + player.w;
    }
}

pub fn make_player(player: &mut Entity) {
    player.draw();
    movement(player);
}

