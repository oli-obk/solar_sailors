use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{controlled::ButtonControlledRange, physics::Physics};

pub(crate) struct Sail {
    left_rope: ButtonControlledRange,
    right_rope: ButtonControlledRange,
    sail_width: ButtonControlledRange,
    sail_body: RigidBodyHandle,
}

impl Sail {
    pub(crate) fn new(
        physics: &mut Physics,
        left_rope: f32,
        right_rope: f32,
        sail_width: f32,
        min_sail_width: f32,
    ) -> Self {
        let mut sail_width = ButtonControlledRange::new(sail_width, KeyCode::W);
        sail_width.min = min_sail_width;

        let sail_body = RigidBodyBuilder::new_dynamic().can_sleep(false).additional_mass(10.0).build();
        let sail_body = physics.add(sail_body);

        Self {
            sail_body,
            left_rope: ButtonControlledRange::new(left_rope, KeyCode::A),
            right_rope: ButtonControlledRange::new(right_rope, KeyCode::D),
            sail_width,
        }
    }
    pub(crate) fn update(&mut self) {
        self.left_rope.update();
        self.right_rope.update();
        self.sail_width.update();
    }
    pub(crate) fn draw(&self, anchor: Vec2, physics: &Physics) {
        let (left_x, left_y, right_x, right_y) = self.rope_positions(anchor);
        // Sail
        draw_line(left_x, left_y, right_x, right_y, 1.0, YELLOW);
        // Ropes
        draw_line(anchor.x, anchor.y, left_x, left_y, 1.0, GRAY);
        draw_line(anchor.x, anchor.y, right_x, right_y, 1.0, GRAY);

        let sail_body = physics.get(self.sail_body);
        let pos = *sail_body.translation();
        let x = pos[0];
        let y = pos[1];
        draw_circle(x, y, 20.0, PINK);
    }

    /// Compute the position of the sail corners
    fn rope_positions(&self, anchor: Vec2) -> (f32, f32, f32, f32) {
        let angle = rope_angle(
            self.left_rope.value,
            self.right_rope.value,
            self.sail_width.value,
        );
        let half_angle = angle / 2.0;
        let x = half_angle.sin();
        let y = half_angle.cos();
        (
            -x * self.left_rope.value + anchor.x,
            -y * self.left_rope.value + anchor.y,
            x * self.right_rope.value + anchor.x,
            -y * self.right_rope.value + anchor.y,
        )
    }
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
