use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{
    controlled::ButtonControlledRange,
    datastructures::{Reader, Sensor},
};

pub(crate) struct Sail {
    pub left_rope: ButtonControlledRange,
    pub right_rope: ButtonControlledRange,
    pub sail_width: ButtonControlledRange,
    /// Computed in the update phase, processed by draw
    rope_positions: Sensor<(Vec2, Vec2)>,
    /// When the sail moves due to different rope lengths, this is all that actually changes.
    /// 0.0 is straight up.
    current_angle: Sensor<f32>,
    pub current_angular_velocity: f32,
    /// The force with which the sail pulls.
    force: Sensor<f32>,
}

const SIDE: f32 = 5.0;

impl Sail {
    pub(crate) fn new(
        left_rope: f32,
        right_rope: f32,
        sail_width: f32,
        min_sail_width: f32,
    ) -> (
        Self,
        Reader<(Vec2, Vec2)>,
        Reader<f32>,
        Reader<f32>,
        Reader<f32>,
        Reader<f32>,
    ) {
        let (sail_width, _) = ButtonControlledRange::new(min_sail_width, sail_width, KeyCode::W);
        let (left_rope, lr) = ButtonControlledRange::new(1.0, left_rope, KeyCode::A);
        let (right_rope, rr) = ButtonControlledRange::new(1.0, right_rope, KeyCode::D);
        let (rope_positions, r2) = Sensor::new(Default::default());
        let (force, f) = Sensor::new(0.0);
        let (current_angle, cur_a) = Sensor::new(0.0);
        (
            Self {
                left_rope,
                right_rope,
                sail_width,
                current_angle,
                current_angular_velocity: 0.0,
                force,
                rope_positions,
            },
            r2,
            f,
            cur_a,
            lr,
            rr,
        )
    }
}

impl crate::ship::Attachement for Sail {
    fn update(&mut self) {
        self.left_rope.update();
        self.right_rope.update();
        self.sail_width.update();

        let (left, right) = self.rope_positions();
        self.rope_positions.set((left, right));

        let vec = left - right;
        let angle = vec.y.atan2(vec.x);
        let cos_angle = angle.cos();
        let f = self.sail_width.value.get() * cos_angle * cos_angle;

        self.force.set(f);

        let dir = self.right_rope.value.get() - self.left_rope.value.get();
        let threshold = 0.001;
        // Only change angular velocity if rope difference is above a threshold.
        let dir = dir.signum() * (dir.abs() - threshold).max(0.0);
        // Scale the velocity acceleration to feel nicer
        let scale = 0.00000001;

        self.current_angular_velocity += dir * f * scale;

        // Add some dampening for the velocity so you don't have to manually
        // control the direction all the time
        if dir == 0.0 || f < 10.0 {
            self.current_angular_velocity *= 0.95;
        }

        let a = self
            .current_angle
            .update(|a| a + self.current_angular_velocity);

        if a.abs() > std::f32::consts::FRAC_PI_6 {
            // Sail uncontrollable
        }
    }

    fn draw(&self, pos: Vec2, angle: f32) {
        // Draw the sail anchor
        {
            let left = -self.sail_width.min / 2.0;
            draw_triangle(
                pos + vec2(left - SIDE, 0.0),
                pos + vec2(left - SIDE, -SIDE),
                pos + vec2(left, 0.0),
                BLUE,
            );
            let right = self.sail_width.min / 2.0;
            draw_triangle(
                pos + vec2(right + SIDE, 0.0),
                pos + vec2(right + SIDE, -SIDE),
                pos + vec2(right, 0.0),
                BLUE,
            );
        }
        let (left, right) = self.rope_positions.get();
        let left = left + pos;
        let right = right + pos;
        // Sail
        draw_line(left, right, 1.0, GOLD);
        // Ropes
        draw_line(pos, left, 1.0, GRAY);
        draw_line(pos, right, 1.0, GRAY);
    }
}

impl Sail {
    /// Compute the position of the sail corners
    pub fn rope_positions(&self) -> (Vec2, Vec2) {
        let angle = rope_angle(
            self.left_rope.value.get(),
            self.right_rope.value.get(),
            self.sail_width.value.get(),
        );
        let half_angle = angle / 2.0;
        let a = self.current_angle.get();
        let left = a + half_angle;
        let right = a - half_angle;
        (
            angle2vec(left) * self.left_rope.value.get(),
            angle2vec(right) * self.right_rope.value.get(),
        )
    }
}

pub fn angle2vec(angle: f32) -> Vec2 {
    let (x, y) = (angle + PI).sin_cos();
    vec2(x, y)
}

pub fn draw_line(a: Vec2, b: Vec2, thickness: f32, color: Color) {
    macroquad::prelude::draw_line(a.x, a.y, b.x, b.y, thickness, color)
}

/// Find the angle between "a" and "b" for a triangle with the given three side lengths.
fn rope_angle(a: f32, b: f32, c: f32) -> f32 {
    // http://mathcentral.uregina.ca/QQ/database/QQ.09.07/h/lucy1.html
    // c^2 = a^2 + b^2 - 2ab cos(C)
    // 2ab cos(C) = a^2 + b^2 - c^2
    // cos(C) = (a^2 + b^2 - c^2)/(2ab)
    let squares = a * a + b * b - c * c;
    let divisor = 2.0 * a * b;
    let cos_c = squares / divisor;
    cos_c.acos()
}
