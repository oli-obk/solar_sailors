use macroquad::prelude::*;

use crate::controlled::ButtonControlledRange;

pub(crate) struct Sail {
    left_rope: ButtonControlledRange,
    right_rope: ButtonControlledRange,
    sail_width: ButtonControlledRange,
    anchor_pos: Vec2,
}

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
        }
    }

    pub(crate) fn update(&mut self) {
        self.left_rope.update();
        self.right_rope.update();
        self.sail_width.update();
    }

    pub(crate) fn draw(&self) {
        let anchor = self.anchor_pos;
        let side = 5.0;
        // Draw the sail anchor
        {
            let left = anchor[0] - self.sail_width.min / 2.0;
            let top = anchor[1] - side;
            draw_rectangle(
                left - side,
                top,
                self.sail_width.min + side * 2.0,
                side,
                BLUE,
            );
            draw_triangle(
                vec2(left - side, top),
                vec2(left - side, top - side),
                vec2(left, top),
                BLUE,
            );
            let right = anchor[0] + self.sail_width.min / 2.0;
            draw_triangle(
                vec2(right + side, top),
                vec2(right + side, top - side),
                vec2(right, top),
                BLUE,
            );
        }
        let anchor = anchor - vec2(0.0, side);
        let (left, right) = self.rope_positions(anchor);
        // Sail
        draw_line(left, right, 1.0, GOLD);
        // Ropes
        draw_line(anchor, left, 1.0, GRAY);
        draw_line(anchor, right, 1.0, GRAY);
    }

    /// Compute the position of the sail corners
    fn rope_positions(&self, anchor: Vec2) -> (Vec2, Vec2) {
        let angle = rope_angle(
            self.left_rope.value,
            self.right_rope.value,
            self.sail_width.value,
        );
        let half_angle = angle / 2.0;
        let x = half_angle.sin();
        let y = half_angle.cos();
        (
            vec2(-x, -y) * self.left_rope.value + anchor,
            vec2(x, -y) * self.right_rope.value + anchor,
        )
    }
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
