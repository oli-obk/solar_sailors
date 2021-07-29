use macroquad::prelude::*;

use crate::{ sail::Sail};

pub(crate) struct SpaceShip {
    pub(crate) pos: Vec2,
    pub(crate) sail: Sail,
    pub(crate) width: f32,
    pub(crate) len: f32,
}

impl SpaceShip {
    pub(crate) fn update(&mut self) {
        self.sail.update();
    }
    pub(crate) fn draw(&self) {
        self.sail.draw();

        // Spaceship
        let mid = self.pos;
        draw_rectangle(mid.x - self.width / 2.0, mid.y, self.width, self.len, BLUE);
    }
}
