use std::f32::consts::PI;

use macroquad::prelude::*;
use macroquad::rand::RandomRange;

use crate::datastructures::Reader;

struct Star {
    pos: Vec2,
    rot: f32,
    size: f32,
}

#[derive(Default)]
pub struct Stars {
    /// Positions and rotations of the stars
    positions: Vec<Star>,
    photons: Vec<Photon>,
    pub sails: Vec<Reader<(Vec2, Vec2)>>,
    w: f32,
    h: f32,
    // Maximum sizes of the window over the runtime of the program
    max_w: f32,
    max_h: f32,
}

pub fn rand_in_rect(rect: Rect) -> Vec2 {
    vec2(
        f32::gen_range(rect.left(), rect.right()),
        f32::gen_range(rect.top(), rect.bottom()),
    )
}

impl Stars {
    pub fn update(&mut self) {
        let w = screen_width();
        let h = screen_height();
        let mut rect = Rect::new(-w / 2.0, -h / 2.0, w, h);
        if w != self.w || h != self.h {
            self.w = w;
            self.h = h;
            let count = (w * h) as usize / 5000;
            if self.w > self.max_w || self.h > self.max_h {
                self.max_h = self.h.max(self.max_h);
                self.max_w = self.w.max(self.max_w);
                // TODO: only add new ones in the newly added screen space
                self.positions = (0..count)
                    .map(|_| Star {
                        pos: rand_in_rect(rect),
                        rot: f32::gen_range(0., PI),
                        size: f32::gen_range(1., 8.),
                    })
                    .collect();
            }
            self.photons = (0..count)
                .map(|_| Photon {
                    pos: rand_in_rect(rect),
                    dir: vec2(0.0, -SPEED),
                })
                .collect();
        }
        rect.x -= SPEED;
        rect.y -= SPEED;
        rect.w += SPEED;
        rect.h += SPEED;
        for photon in &mut self.photons {
            let len_vec = photon.dir.normalize() * LENGTH;
            for sail in &self.sails {
                let (l, r) = sail.get().unwrap();
                if let Some(collision) = intersect((photon.pos, len_vec), (l, r - l)) {
                    let sail_dir = (r - l).normalize();
                    photon.pos = collision;
                    photon.dir = -photon.dir - 2.0 * (-photon.dir).dot(sail_dir) * sail_dir;
                }
            }
            photon.pos += photon.dir;
            if !rect.contains(photon.pos) {
                photon.pos = vec2(f32::gen_range(rect.left(), rect.right()), rect.bottom());
                photon.dir = vec2(0.0, -SPEED);
            }
        }
    }
    pub fn draw(&self) {
        for star in &self.positions {
            star.draw();
        }

        for photon in &self.photons {
            draw_line(
                photon.pos.x,
                photon.pos.y,
                photon.pos.x + photon.dir.x * LENGTH / SPEED,
                photon.pos.y + photon.dir.y * LENGTH / SPEED,
                1.0,
                GOLD.with_alpha(0.2),
            );
        }
    }
}

impl Star {
    fn draw(&self) {
        let y = f32::sin(std::f32::consts::PI / 3.0) * self.size;
        let x = self.size / 2.0;
        let rot = Vec2::from_angle(self.rot);
        let left = rot.rotate(Vec2::new(-x, y));
        let right = rot.rotate(Vec2::new(x, y));
        draw_triangle(self.pos, self.pos + left, self.pos + right, WHITE)
    }
}

#[test]
fn intersect_check() {
    let test = intersect(
        (vec2(2.0, 5.0), vec2(4.0, -5.0)),
        (vec2(1.0, 1.0), vec2(6.0, 1.0)),
    );
    assert_eq!(
        test.map(|f| f.to_string()),
        Some("[4.705882, 1.6176472]".to_string())
    );
}

pub(crate) struct Photon {
    pub(crate) pos: Vec2,
    pub(crate) dir: Vec2,
}

pub(crate) const SPEED: f32 = 2.0;

/// Find the intersection point of two lines specified by its starting and end point, if there is one.
pub(crate) fn intersect(line1: (Vec2, Vec2), line2: (Vec2, Vec2)) -> Option<Vec2> {
    let starting_point_diff = line2.0 - line1.0;
    let line1_intersect_factor = starting_point_diff.perp_dot(line2.1) / line1.1.perp_dot(line2.1);
    let pos = line1.0 + line1.1 * line1_intersect_factor;
    let line2_intersect_vec = (pos - line2.0) / line2.1;
    if (0.0..=1.0).contains(&line1_intersect_factor)
        // FIXME: Why 2.0???? otherwise only the left half of the sail is hit.
        && line2_intersect_vec.length_squared() < 2.0
        && line2_intersect_vec.x >= 0.0
        && line2_intersect_vec.y >= 0.0
    {
        Some(pos)
    } else {
        None
    }
}

pub(crate) const LENGTH: f32 = 10.0;
