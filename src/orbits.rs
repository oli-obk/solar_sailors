pub use ::orbits::*;
use macroquad::prelude::*;

use crate::save::Saveable;

pub struct Orbits {
    pub orbits: orbits::Orbits,
    pub t: Saveable<f64>,
}

pub struct ObjectId(#[expect(dead_code)] usize);

const MOON_SIZE: f32 = 20.0;

impl Orbits {
    pub fn load() -> Self {
        Self {
            orbits: Default::default(),
            t: Saveable::default("time"),
        }
    }
    pub fn insert(&mut self, object: orbits::Object) -> ObjectId {
        ObjectId(self.orbits.insert(object))
    }
    pub fn update(&mut self) {
        self.t += 10.0;
        // only need to do something for objects under thrust
    }
    pub fn draw(&self) {
        for (kind, pos, mut points) in self.orbits.draw(*self.t, 100) {
            let pos = Vec2::from(pos);
            let size = 10.0;
            let y = f32::sin(std::f32::consts::PI / 3.0) * size;
            let x = size / 2.0;
            let left = Vec2::new(-x, y);
            let right = Vec2::new(x, y);
            draw_triangle(pos, pos + left, pos + right, GREEN);

            let color = match kind {
                OrbitKind::Circle => WHITE,
                OrbitKind::Ellipse => GRAY,
                OrbitKind::Parabola => GREEN,
                OrbitKind::Hyperbola => RED,
            };

            let (mut x, mut y) = points.next().unwrap();

            for (new_x, new_y) in points {
                draw_line(x, y, new_x, new_y, 0.5, color);
                x = new_x;
                y = new_y;
            }
        }
        draw_circle(0.0, 0.0, MOON_SIZE, GRAY);
        draw_rectangle(
            -MOON_SIZE,
            0.0,
            MOON_SIZE * 2.0,
            -1000.0,
            Color::new(0.0, 0.0, 0.0, 0.5),
        );
    }
}
