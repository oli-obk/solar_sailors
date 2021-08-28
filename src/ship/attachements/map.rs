use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;

use crate::ship::{SIZE, SQRT3};

pub struct Map {
    pub texture: Texture2D,
    pub zoom: f32,
    pub small_zoom: f32,
}

impl crate::ship::Attachement for Map {
    fn update(&mut self, _pos: macroquad::prelude::Vec2, _angle: f32) {}

    fn control(&mut self, dir: Option<bool>, x: f32) {
        if let Some(dir) = dir {
            if dir {
                self.zoom *= 1.01;
            } else {
                self.zoom /= 1.01;
            }
        } else {
            let new = 3.0 - (x.abs() / 10.0).min(2.0);
            self.small_zoom += (new - self.small_zoom) * 0.2;
        }
    }

    fn draw(&self, mut pos: macroquad::prelude::Vec2, angle: f32) {
        let (y, x) = (angle - FRAC_PI_2).sin_cos();
        let zoom = self.zoom * self.small_zoom;
        pos += vec2(x, y) * 1024.0 / 2.0 * zoom;
        pos -= vec2(self.texture.width(), self.texture.height()) / 2.0 * zoom;
        draw_texture_ex(
            self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(1024.0, 1024.0) * zoom),
                ..DrawTextureParams::default()
            },
        );
    }

    fn draw_controllable(&self, _pos: macroquad::prelude::Vec2, _x: f32) {}
}
