use macroquad::prelude::*;

use crate::{physics::Physics, sail::Sail};

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
    pub(crate) fn draw(&self, physics: &Physics) {
        self.sail.draw(self.pos, physics);

        // Spaceship
        let mid = self.pos;
        draw_triangle(
            mid,
            mid + Vec2::new(10.0, self.width / 2.0),
            mid + Vec2::new(-10.0, self.width / 2.0),
            BLUE,
        );
        draw_rectangle(
            mid.x - self.width / 2.0,
            mid.y + 10.0,
            self.width,
            self.len,
            BLUE,
        );
    }
}
