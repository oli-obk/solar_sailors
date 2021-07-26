use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{controlled::ButtonControlledRange, physics::Physics};

pub(crate) struct Sail {
    left_rope: ButtonControlledRange,
    right_rope: ButtonControlledRange,
    sail_width: ButtonControlledRange,
    sail: RigidBodyHandle,
    anchor: RigidBodyHandle,
    left_rope_joints: Vec<JointHandle>,
    right_rope_joints: Vec<JointHandle>,
}

impl Sail {
    pub(crate) fn new(
        physics: &mut Physics,
        left_rope: f32,
        right_rope: f32,
        sail_width: f32,
        min_sail_width: f32,
    ) -> Self {
        let anchor = RigidBodyBuilder::new_static()
            .translation(vector![100.0 + sail_width / 2.0, 200.0])
            .build();
        let anchor = physics.add(anchor);

        let sail = RigidBodyBuilder::new_dynamic()
            .can_sleep(false)
            .additional_mass(0.01)
            .additional_principal_angular_inertia(1.0)
            .translation(vector![100.0 + sail_width / 2.0, 100.0])
            .build();

        let sail = physics.add(sail);

        let mut left_rope_joints = Vec::new();
        let mut right_rope_joints = Vec::new();
        for (rope_length, rope, offset) in &mut [
            (left_rope, &mut left_rope_joints, -sail_width / 2.0),
            (right_rope, &mut right_rope_joints, sail_width / 2.0),
        ] {
            // Shorter segments are more rope-y, but also accumulate more error
            let segment_len = 5;
            let mut connect_nodes = |physics: &mut Physics, body1, body2| {
                let segment = BallJoint::new(point![0.0, -(segment_len as f32)], point![0.0, 0.0]);
                rope.push(physics.add_joint(segment, body1, body2));
            };

            let mut prev_node = anchor;
            let mut mk_segment = |x, y| {
                let next_node = RigidBodyBuilder::new_dynamic()
                    .can_sleep(false)
                    .additional_mass(0.01 * (segment_len as f32))
                    .additional_principal_angular_inertia(0.00001 * (segment_len as f32))
                    .gravity_scale(0.0)
                    .translation(vector![x, y])
                    .build();
                let next_node = physics.add(next_node);
                connect_nodes(physics, prev_node, next_node);
                prev_node = next_node;
            };
            let rope_length = *rope_length;
            for i in (0..(rope_length as usize + 1)).step_by(segment_len) {
                mk_segment(
                    100.0 + sail_width / 2.0 + *offset * (i as f32) / rope_length,
                    200.0 - 100.0 * (i as f32) / rope_length,
                );
            }
            let segment = BallJoint::new(point![*offset, 0.0], point![0.0, 0.0]);
            rope.push(physics.add_joint(segment, sail, prev_node));
        }

        let mut sail_width = ButtonControlledRange::new(sail_width, KeyCode::W);
        sail_width.min = min_sail_width;

        Self {
            left_rope: ButtonControlledRange::new(left_rope, KeyCode::A),
            right_rope: ButtonControlledRange::new(right_rope, KeyCode::D),
            sail_width,
            sail,
            anchor,
            left_rope_joints,
            right_rope_joints,
        }
    }
    pub(crate) fn update(&mut self, physics: &mut Physics) {
        self.left_rope.update();
        self.right_rope.update();
        self.sail_width.update();

        // Resize the sail
        for (rope, dir) in [
            (&self.right_rope_joints, 1.0),
            (&self.left_rope_joints, -1.0),
        ] {
            // Last rope segment is the connection to the sail
            let rope = &mut physics[*rope.last().unwrap()];
            if let JointParams::BallJoint(joint) = &mut rope.params {
                // We only modify the x corrdinate of the anchor at the sail
                let x = &mut joint.local_anchor1.coords[0];
                // Compute the difference between the desired sail size and the actual sail size
                let diff = *x - dir * self.sail_width.value / 2.0;
                let step = 0.01;
                if diff.abs() >= step {
                    *x -= diff.signum() * step;
                }
            }
        }
    }
    pub(crate) fn draw(&self, anchor: Vec2, physics: &Physics) {
        let (left_x, left_y, right_x, right_y) = self.rope_positions(anchor);
        // Sail
        draw_line(left_x, left_y, right_x, right_y, 1.0, YELLOW);
        // Ropes
        draw_line(anchor.x, anchor.y, left_x, left_y, 1.0, GRAY);
        draw_line(anchor.x, anchor.y, right_x, right_y, 1.0, GRAY);

        // Draws a single rope segment
        let draw_joint = |joint: JointHandle, color| {
            let Joint { body1, body2, .. } = physics[joint];
            let pos = physics[body1].translation();
            let x1 = pos[0];
            let y1 = pos[1];
            let pos = physics[body2].translation();
            let x2 = pos[0];
            let y2 = pos[1];
            draw_line(x1, y1, x2, y2, 1.0, color);
        };

        // Draw the rope
        for &rope in &[&self.right_rope_joints, &self.left_rope_joints] {
            for &joint in rope.iter().rev().skip(1).rev() {
                draw_joint(joint, GRAY);
            }
        }

        // Draw the sail
        let left = &physics[*self.left_rope_joints.last().unwrap()];
        let joint = left.params.as_ball_joint().unwrap().local_anchor1;
        let left = physics[left.body1].position().transform_point(&joint);
        let right = &physics[*self.right_rope_joints.last().unwrap()];
        let joint = right.params.as_ball_joint().unwrap().local_anchor1;
        let right = physics[right.body1].position().transform_point(&joint);

        draw_line(left[0], left[1], right[0], right[1], 1.0, GOLD);
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
