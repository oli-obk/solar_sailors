use macroquad::prelude::*;
use macroquad::rand::RandomRange;

use crate::stars::rand_in_rect;

pub struct PhotonMap {
    photons: Vec<Photon>,
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

        Self { photons, rect }
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

impl PhotonMap {
    pub fn update(&mut self, (l, r): (Vec2, Vec2)) {
        for photon in &mut self.photons {
            if let Some(_collision) = intersect((photon.pos, photon.dir), (l, r - l)) {
                let sail_dir = (r - l).normalize();
                photon.dir = -photon.dir - 2.0 * (-photon.dir).dot(sail_dir) * sail_dir;
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
                photon.pos.x + photon.dir.x * 10.0 / SPEED,
                photon.pos.y + photon.dir.y * 10.0 / SPEED,
                1.0,
                GOLD,
            );
        }
    }
}
