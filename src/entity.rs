use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Entity{
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }

    pub fn draw_circle(&self, r: f32) {
        draw_poly(self.x, self.y, 255, r,0., self.color);
    }

    pub fn intersects(&self, other: &Entity) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}
