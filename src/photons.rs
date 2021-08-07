use macroquad::prelude::*;
use macroquad::rand::RandomRange;

use crate::stars::rand_in_rect;

pub struct PhotonMap {
    photons: Vec<Photon>,
    rect: Rect,
}

impl PhotonMap {
    pub fn new(count: usize, rect: Rect) -> Self {
        let photons = (0..count)
            .map(|_| Photon {
                pos: rand_in_rect(rect),
                dir: vec2(0.0, -SPEED),
            })
            .collect();
        Self { photons, rect }
    }
}

struct Photon {
    pos: Vec2,
    dir: Vec2,
}

const SPEED: f32 = 5.0;

impl PhotonMap {
    pub fn update(&mut self) {
        for photon in &mut self.photons {
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
