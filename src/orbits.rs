use std::{collections::HashMap, convert::TryFrom as _, f64::consts::TAU};

use macroquad::prelude::*;
use orbital::typed_floats::{NonNaNFinite, PositiveFinite};

use crate::save::Saveable;

struct Orbit {
    /// Angle of apehelion.
    angle: NonNaNFinite,
    /// Starting point of object in the orbit.
    t: PositiveFinite,
    /// raw orbit information.
    orbit: orbital::Orbit,
}

struct Object {
    orbit: Orbit,
}

pub struct Orbits {
    objects: HashMap<usize, Object>,
    next_id: usize,
    pub t: Saveable<f64>,
}

pub struct ObjectId(#[allow(dead_code)] usize);

const MOON_SIZE: f32 = 20.0;

impl Orbits {
    pub fn load() -> Self {
        Self {
            objects: Default::default(),
            next_id: 0,
            t: Saveable::default("time"),
        }
    }
    pub fn insert(
        &mut self,
        angle: NonNaNFinite,
        orbit: orbital::Orbit,
        t: PositiveFinite,
    ) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(
            id,
            Object {
                orbit: Orbit { angle, orbit, t },
            },
        );
        ObjectId(id)
    }
    pub fn update(&mut self) {
        self.t += 10.0;
        // only need to do something for objects under thrust
    }
    pub fn draw(&self) {
        for object in self.objects.values() {
            let angle = object.orbit.orbit.angle_at(
                PositiveFinite::try_from(
                    PositiveFinite::try_from(*self.t).unwrap() + object.orbit.t,
                )
                .unwrap(),
            );
            let radius = object.orbit.orbit.r(angle);
            let system_angle = f64::from(angle + object.orbit.angle);
            let y = system_angle.sin();
            let x = system_angle.cos();
            let x = x as f32;
            let y = y as f32;
            let pos = vec2(x, y) * (f64::from(radius) as f32);
            let size = 10.0;
            let y = f32::sin(std::f32::consts::PI / 3.0) * size;
            let x = size / 2.0;
            let left = Vec2::new(-x, y);
            let right = Vec2::new(x, y);
            draw_triangle(pos, pos + left, pos + right, GREEN);

            let segments = 100;
            let (start, range) = match object.orbit.orbit.kind() {
                orbital::OrbitKind::Circle | orbital::OrbitKind::Ellipse => (0.0, TAU),
                orbital::OrbitKind::Parabola | orbital::OrbitKind::Hyperbola => {
                    // 1/e = cos(angle)
                    let angle = (-1.0 / f64::from(object.orbit.orbit.epsilon)).acos();
                    let range = angle * 2.0;
                    // Subtract one degree so we don't render over infinity.
                    (-angle + TAU / 360.0, range - TAU / 180.0)
                }
            };
            let (y, x) = start.sin_cos();
            let mut x = x as f32;
            let mut y = y as f32;
            let step_size = range / segments as f64;
            let r = f64::from(
                object
                    .orbit
                    .orbit
                    .r(NonNaNFinite::try_from(start + f64::from(object.orbit.angle)).unwrap()),
            ) as f32;
            y *= r;
            x *= r;
            let color = match object.orbit.orbit.kind() {
                orbital::OrbitKind::Circle => WHITE,
                orbital::OrbitKind::Ellipse => GRAY,
                orbital::OrbitKind::Parabola => GREEN,
                orbital::OrbitKind::Hyperbola => RED,
            };
            for i in 0..segments {
                let angle = step_size * (i + 1) as f64 + start;
                let (new_y, new_x) = angle.sin_cos();
                let mut new_x = new_x as f32;
                let mut new_y = new_y as f32;
                let r = f64::from(
                    object
                        .orbit
                        .orbit
                        .r(NonNaNFinite::try_from(angle - f64::from(object.orbit.angle)).unwrap()),
                ) as f32;
                new_y *= r;
                new_x *= r;
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
