use std::cell::RefCell;
use std::rc::Weak;

use macroquad::prelude::*;
use macroquad::rand::RandomRange;

use crate::stars::rand_in_rect;

pub struct PhotonMap {
    photons: Vec<Photon>,
    pub sails: Vec<Weak<RefCell<(Vec2, Vec2)>>>,
    rect: Rect,
}

impl PhotonMap {
    pub fn new(count: usize, mut rect: Rect) -> Self {
        let photons = (0..count)
            .map(|_| Photon {
                pos: rand_in_rect(rect),
                dir: vec2(0.0, -SPEED),
            })
            .collect();
        rect.x -= SPEED;
        rect.y -= SPEED;
        rect.w += SPEED;
        rect.h += SPEED;
        let test = intersect(
            (vec2(2.0, 5.0), vec2(4.0, -5.0)),
            (vec2(1.0, 1.0), vec2(6.0, 1.0)),
        );
        assert_eq!(
            test.map(|f| f.to_string()),
            Some("[4.705882, 1.6176472]".to_string())
        );

        Self {
            photons,
            rect,
            sails: Default::default(),
        }
    }
}

struct Photon {
    pos: Vec2,
    dir: Vec2,
}

const SPEED: f32 = 2.0;

/// Find the intersection point of two lines specified by its starting and end point, if there is one.
fn intersect(line1: (Vec2, Vec2), line2: (Vec2, Vec2)) -> Option<Vec2> {
    let starting_point_diff = line2.0 - line1.0;
    let line1_intersect_factor = starting_point_diff.perp_dot(line2.1) / line1.1.perp_dot(line2.1);
    let pos = line1.0 + line1.1 * line1_intersect_factor;
    let line2_intersect_vec = (pos - line2.0) / line2.1;
    if line1_intersect_factor >= 0.0
        && line1_intersect_factor <= 1.0
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

const LENGTH: f32 = 10.0;

impl PhotonMap {
    pub fn update(&mut self) {
        for photon in &mut self.photons {
            let len_vec = photon.dir.normalize() * LENGTH;
            for sail in &self.sails {
                let (l, r) = *sail.upgrade().unwrap().borrow();
                if let Some(collision) = intersect((photon.pos, len_vec), (l, r - l)) {
                    let sail_dir = (r - l).normalize();
                    photon.pos = collision;
                    photon.dir = -photon.dir - 2.0 * (-photon.dir).dot(sail_dir) * sail_dir;
                }
            }
            photon.pos += photon.dir;
            if !self.rect.contains(photon.pos) {
                photon.pos = vec2(
                    f32::gen_range(self.rect.left(), self.rect.right()),
                    self.rect.bottom(),
                );
                photon.dir = vec2(0.0, -SPEED);
            }
        }
    }

    pub fn draw(&self) {
        for photon in &self.photons {
            draw_line(
                photon.pos.x,
                photon.pos.y,
                photon.pos.x + photon.dir.x * LENGTH / SPEED,
                photon.pos.y + photon.dir.y * LENGTH / SPEED,
                1.0,
                GOLD,
            );
        }
    }
}
