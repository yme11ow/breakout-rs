use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Entity {
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}
