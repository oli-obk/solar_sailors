use std::{collections::HashMap, f32::consts::TAU};

use macroquad::prelude::*;

use crate::save::Saveable;

struct Orbit {
    angle: f32,
    orbit: orbital::Orbit,
}

struct Object {
    orbit: Orbit,
}

pub struct Orbits {
    objects: HashMap<usize, Object>,
    next_id: usize,
    t: Saveable<f32>,
}

pub struct ObjectId(usize);

const MOON_SIZE: f32 = 20.0;

impl Orbits {
    pub fn load() -> Self {
        Self {
            objects: Default::default(),
            next_id: 0,
            t: Saveable::default("time"),
        }
    }
    pub fn insert(&mut self, angle: f32, orbit: orbital::Orbit) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(
            id,
            Object {
                orbit: Orbit { angle, orbit },
            },
        );
        ObjectId(id)
    }
    pub fn update(&mut self) {
        self.t.update(|t| *t += 10.0);
        // only need to do something for objects under thrust
    }
    pub fn draw(&self) {
        for object in self.objects.values() {
            let angle = object.orbit.orbit.angle_at(1.0, *self.t);
            let radius = object.orbit.orbit.r(angle);
            let system_angle = angle + object.orbit.angle;
            let (y, x) = system_angle.sin_cos();
            let pos = vec2(x, y) * radius;
            let size = 10.0;
            let y = f32::sin(std::f32::consts::PI / 3.0) * size;
            let x = size / 2.0;
            let left = Vec2::new(-x, y);
            let right = Vec2::new(x, y);
            draw_triangle(pos, pos + left, pos + right, GREEN);

            let segments = 100;
            let (mut y, mut x) = (-object.orbit.angle).sin_cos();
            let r = object.orbit.orbit.r(-object.orbit.angle);
            y *= r;
            x *= r;
            let step_size = TAU / segments as f32;
            for i in 0..segments {
                let angle = step_size * (i + 1) as f32 - object.orbit.angle;
                let (mut new_y, mut new_x) = angle.sin_cos();
                let r = object.orbit.orbit.r(angle);
                new_y *= r;
                new_x *= r;
                draw_line(x, y, new_x, new_y, 0.5, GRAY);
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
