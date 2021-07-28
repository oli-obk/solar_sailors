use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{controlled::ButtonControlledRange, physics::Physics};

pub(crate) struct Sail {
    sail_width: ButtonControlledRange,
    sail: RigidBodyHandle,
    ropes: [Rope; 2],
    anchor_pos: Vector<Real>,
}

struct Rope {
    controller: ButtonControlledRange,
    joints: Vec<JointHandle>,
}

impl Sail {
    pub(crate) fn new(
        physics: &mut Physics,
        sail_width: f32,
        min_sail_width: f32,
        anchor_pos: Vector<Real>,
    ) -> Self {
        let rope_length = 100;

        let sail = RigidBodyBuilder::new_dynamic()
            .can_sleep(false)
            .additional_mass(0.1)
            .additional_principal_angular_inertia(1.0)
            .translation(anchor_pos + vector![0.0, -(rope_length as f32)])
            .build();

        let sail = physics.add(sail);

        let mut ropes = [
            Rope {
                controller: ButtonControlledRange::new(100.0, KeyCode::A),
                joints: Vec::new(),
            },
            Rope {
                controller: ButtonControlledRange::new(100.0, KeyCode::D),
                joints: Vec::new(),
            },
        ];
        for (rope, dir) in ropes.iter_mut().zip([-1.0, 1.0]) {
            let offset = sail_width / 2.0 * dir;
            let y_offset = min_sail_width / 2.0 * dir;
            let mut connect_nodes = |physics: &mut Physics, body1, body2| {
                let segment = BallJoint::new(point![0.0, -1.0], point![0.0, 0.0]);
                rope.joints.push(physics.add_joint(segment, body1, body2));
            };

            let anchor_pos = anchor_pos + vector![y_offset, -5.0];
            let anchor = RigidBodyBuilder::new_static()
                .translation(anchor_pos)
                .build();
            let anchor = physics.add(anchor);
            let mut prev_node = anchor;
            let mut mk_segment = |pos| {
                let next_node = RigidBodyBuilder::new_dynamic()
                    .can_sleep(false)
                    .additional_mass(0.001)
                    .additional_principal_angular_inertia(0.00001)
                    .gravity_scale(0.0)
                    .translation(pos)
                    .build();
                let next_node = physics.add(next_node);
                connect_nodes(physics, prev_node, next_node);
                prev_node = next_node;
            };
            for i in 0..rope_length {
                let rope_length = rope_length as f32;
                let frac = (i as f32) / rope_length;
                mk_segment(anchor_pos + vector![offset * frac, -rope_length * frac]);
            }
            let segment = BallJoint::new(point![offset, 0.0], point![0.0, 0.0]);
            rope.joints
                .push(physics.add_joint(segment, sail, prev_node));
        }

        let mut sail_width = ButtonControlledRange::new(sail_width, KeyCode::W);
        sail_width.min = min_sail_width;

        Self {
            sail_width,
            sail,
            ropes,
            anchor_pos,
        }
    }
    pub(crate) fn update(&mut self, physics: &mut Physics) {
        // Resize the sail and apply the photon pressure
        for (rope, dir) in self.ropes.iter().zip([-0.5, 0.5]) {
            // Last rope segment is the connection to the sail
            let rope = &mut physics[*rope.joints.last().unwrap()];
            if let JointParams::BallJoint(joint) = &mut rope.params {
                // We only modify the x coordinate of the anchor at the sail
                let x = &mut joint.local_anchor1.coords[0];
                // Compute the difference between the desired sail size and the actual sail size

                let step = 0.01;
                *x = self.sail_width.apply(*x / dir, step) * dir;

                // Apply force only once
                if dir > 0.0 {
                    // Don't care if I'm left or right
                    let x = x.abs();
                    // The area that photons hit.
                    let sail = &mut physics[self.sail];
                    let sail_volume = sail.rotation().transform_vector(&vector![1.0, 0.0])[0];
                    let force = sail
                        .rotation()
                        .transform_vector(&vector![0.0, -sail_volume * x * 0.01]);
                    sail.apply_force(force, true);
                }
            }
        }
    }

    pub(crate) fn draw(&self, physics: &Physics) {
        // Draw the sail anchor
        {
            let side = 5.0;
            let left = self.anchor_pos[0] - self.sail_width.min / 2.0;
            let top = self.anchor_pos[1] - side;
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
            let right = self.anchor_pos[0] + self.sail_width.min / 2.0;
            draw_triangle(
                vec2(right + side, top),
                vec2(right + side, top - side),
                vec2(right, top),
                BLUE,
            );
        }

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
        for rope in &self.ropes {
            for &joint in rope.joints.iter().rev().skip(1).rev() {
                draw_joint(joint, GRAY);
            }
        }

        // Draw the sail
        let left = &physics[*self.ropes[0].joints.last().unwrap()];
        let joint = left.params.as_ball_joint().unwrap().local_anchor1;
        let left = physics[left.body1].position().transform_point(&joint);
        let right = &physics[*self.ropes[1].joints.last().unwrap()];
        let joint = right.params.as_ball_joint().unwrap().local_anchor1;
        let right = physics[right.body1].position().transform_point(&joint);

        draw_line(left[0], left[1], right[0], right[1], 1.0, GOLD);
    }
}
