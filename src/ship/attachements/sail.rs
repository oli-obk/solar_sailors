use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{
    controlled::ControlledRange,
    datastructures::{Reader, Sensor, SetGet},
    ship::SIZE,
};

pub(crate) struct Sail {
    pub left_rope: ControlledRange,
    pub right_rope: ControlledRange,
    pub sail_width: ControlledRange,
    /// Computed in the update phase, processed by draw
    rope_positions: Sensor<(Vec2, Vec2)>,
    /// When the sail moves due to different rope lengths, this is all that actually changes.
    /// 0.0 is straight up.
    current_angle: Sensor<f32>,
    pub current_angular_velocity: f32,
    /// The force with which the sail pulls.
    force: Sensor<f32>,

    /// Thickness of the helper lines showing what is being controlled
    helper_line: f32,
    helper_pos: Option<f32>,
}

const SIDE: f32 = SIZE / 8.0;

pub(crate) struct SailParameters {
    pub rope_positions: Reader<(Vec2, Vec2)>,
    pub force: Reader<f32>,
    pub current_angle: Reader<f32>,
    pub left_rope: Reader<f32>,
    pub right_rope: Reader<f32>,
}

impl Sail {
    pub(crate) fn new(
        left_rope: f32,
        right_rope: f32,
        sail_width: f32,
        min_sail_width: f32,
        current_angle: f32,
    ) -> (Self, SailParameters) {
        let (sail_width, _) = ControlledRange::new(min_sail_width, sail_width);
        let (left_rope, lr) = ControlledRange::new(1.0, left_rope);
        let (right_rope, rr) = ControlledRange::new(1.0, right_rope);
        let (rope_positions, r2) = Sensor::new(Default::default());
        let (force, f) = Sensor::new(0.0);
        let (current_angle, cur_a) = Sensor::new(current_angle);
        (
            Self {
                left_rope,
                right_rope,
                sail_width,
                current_angle,
                current_angular_velocity: 0.0,
                force,
                rope_positions,
                helper_line: 0.0,
                helper_pos: None,
            },
            SailParameters {
                rope_positions: r2,
                force: f,
                current_angle: cur_a,
                left_rope: lr,
                right_rope: rr,
            },
        )
    }
}

impl crate::ship::Attachement for Sail {
    fn update(&mut self, pos: Vec2, _angle: f32) {
        let (left, right) = self.rope_positions();
        self.rope_positions.set((left + pos, right + pos));

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
        let scale = 0.000000001;

        self.current_angular_velocity += dir * f * scale;

        let threshold = 0.0005 / scale;

        // Add some dampening for the velocity so you don't have to manually
        // control the direction all the time
        println!(
            "{} * {threshold} = {}",
            self.current_angular_velocity,
            self.current_angular_velocity * threshold
        );
        if self.current_angular_velocity.abs() * threshold < 1.0 {
            self.current_angular_velocity *= 0.95;
        }

        let cav = self.current_angular_velocity;
        let a = self.current_angle.modify(|a| a + cav);

        if a.abs() > std::f32::consts::FRAC_PI_6 {
            // Sail uncontrollable
        }
    }

    fn draw(&self, pos: Vec2, angle: f32) {
        // Draw the sail anchor
        {
            let left = -self.sail_width.min / 2.0;
            let (sin, cos) = angle.sin_cos();
            let rot = |coord: Vec2| {
                // p'x = cos(theta) * (px-ox) - sin(theta) * (py-oy) + ox
                let x = cos * coord.x - sin * coord.y;
                let y = sin * coord.x + cos * coord.y;
                pos + vec2(x, y)
            };
            draw_triangle(
                rot(vec2(left - SIDE, 0.0)),
                rot(vec2(left - SIDE, -SIDE)),
                rot(vec2(left, 0.0)),
                BLUE,
            );
            let right = self.sail_width.min / 2.0;
            draw_triangle(
                rot(vec2(right + SIDE, 0.0)),
                rot(vec2(right + SIDE, -SIDE)),
                rot(vec2(right, 0.0)),
                BLUE,
            );
        }
        let (left, right) = self.rope_positions.get();
        // Sail
        draw_line(left, right, 1.0, GOLD);
        // Ropes
        draw_line(pos, left, 1.0, GRAY);
        draw_line(pos, right, 1.0, GRAY);

        let thickness = self.helper_line.abs() * 2.0;
        if let Some(helper_pos) = self.helper_pos {
            if helper_pos.abs() > self.sail_width.min / 2.0 {
                if helper_pos > 0.0 {
                    draw_line(pos, right, thickness, GREEN);
                } else {
                    draw_line(pos, left, thickness, GREEN);
                }
            } else {
                draw_line(left, right, thickness, GREEN);
            }
        }
    }

    fn control(&mut self, dir: Option<bool>, pos: Option<f32>) {
        if let Some(pos) = pos {
            if let Some(dir) = dir {
                let controlled = if pos.abs() > self.sail_width.min / 2.0 {
                    if pos > 0.0 {
                        &mut self.right_rope
                    } else {
                        &mut self.left_rope
                    }
                } else {
                    &mut self.sail_width
                };
                controlled.control(dir)
            }

            self.helper_line += 0.03;
            if self.helper_line > 1.0 {
                self.helper_line -= 2.0;
            }
        }
        self.helper_pos = pos;
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

fn draw_line(a: Vec2, b: Vec2, thickness: f32, color: Color) {
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
