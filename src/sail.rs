use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::controlled::ButtonControlledRange;

pub(crate) struct Sail {
    pub left_rope: ButtonControlledRange,
    pub right_rope: ButtonControlledRange,
    pub sail_width: ButtonControlledRange,
    anchor_pos: Vec2,
    /// Computed in the update phase, processed by draw
    rope_positions: (Vec2, Vec2),
    /// When the sail moves due to different rope lengths, this is all that actually changes.
    /// 0.0 is straight up.
    pub current_angle: f32,
    pub current_angular_velocity: f32,
    /// The force with which the sail pulls.
    pub force: f32,
}

const SIDE: f32 = 5.0;

impl Sail {
    pub(crate) fn new(
        left_rope: f32,
        right_rope: f32,
        sail_width: f32,
        min_sail_width: f32,
        anchor_pos: Vec2,
    ) -> Self {
        let sail_width = ButtonControlledRange::new(min_sail_width, sail_width, KeyCode::W);

        Self {
            left_rope: ButtonControlledRange::new(1.0, left_rope, KeyCode::A),
            right_rope: ButtonControlledRange::new(1.0, right_rope, KeyCode::D),
            sail_width,
            anchor_pos,
            current_angle: 0.0,
            current_angular_velocity: 0.0,
            force: 0.0,
            rope_positions: Default::default(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.left_rope.update();
        self.right_rope.update();
        self.sail_width.update();

        let (left, right) = self.rope_positions();
        self.rope_positions = (left, right);
        // Shift the positions into an anchor-centric system, since we
        // don't care about the real position, but only about the forces.
        let left = left + vec2(0.0, SIDE);
        let right = right + vec2(0.0, SIDE);

        let vec = left - right;
        let angle = vec.y.atan2(vec.x);
        let cos_angle = angle.cos();
        let f = self.sail_width.value * cos_angle * cos_angle;

        self.force = f;

        let dir = self.right_rope.value - self.left_rope.value;
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

        self.current_angle += self.current_angular_velocity;

        if self.current_angle.abs() > std::f32::consts::FRAC_PI_6 {
            // Sail uncontrollable
        }
    }

    pub(crate) fn draw(&self) {
        let anchor = self.anchor_pos;
        // Draw the sail anchor
        {
            let left = anchor[0] - self.sail_width.min / 2.0;
            let top = anchor[1] - SIDE;
            draw_rectangle(
                left - SIDE,
                top,
                self.sail_width.min + SIDE * 2.0,
                SIDE,
                BLUE,
            );
            draw_triangle(
                vec2(left - SIDE, top),
                vec2(left - SIDE, top - SIDE),
                vec2(left, top),
                BLUE,
            );
            let right = anchor[0] + self.sail_width.min / 2.0;
            draw_triangle(
                vec2(right + SIDE, top),
                vec2(right + SIDE, top - SIDE),
                vec2(right, top),
                BLUE,
            );
        }
        let (left, right) = self.rope_positions;
        let offset = vec2(0.0, SIDE);
        let left = left - offset;
        let right = right - offset;
        let anchor = self.anchor_pos - offset;
        // Sail
        draw_line(left, right, 1.0, GOLD);
        // Ropes
        draw_line(anchor, left, 1.0, GRAY);
        draw_line(anchor, right, 1.0, GRAY);
    }

    /// Compute the position of the sail corners
    pub fn rope_positions(&self) -> (Vec2, Vec2) {
        let angle = rope_angle(
            self.left_rope.value,
            self.right_rope.value,
            self.sail_width.value,
        );
        let half_angle = angle / 2.0;
        let left = self.current_angle + half_angle;
        let right = self.current_angle - half_angle;
        (
            angle2vec(left) * self.left_rope.value,
            angle2vec(right) * self.right_rope.value,
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
