use std::{collections::HashMap, f32::consts::PI};

use hex2d::Spacing;
use macroquad::prelude::*;

mod segment;
mod segments {
    mod gauge;
    pub use gauge::{Gauge, GaugeHandle, GaugeHandleKind};
}

mod attachements {
    mod map;
    mod sail;
    pub use map::*;
    pub use sail::*;
}

pub use attachements::*;
pub use segment::*;
pub use segments::*;

pub(crate) struct SpaceShip {
    pub(crate) pos: Vec2,
    pub(crate) grid: HashMap<hex2d::Coordinate, Segment>,
}

// sqrt is not const fn, so we inline sqrt(3)
pub const SQRT3: f32 = 1.732_050_8;
pub(crate) const SPACING: Spacing = Spacing::FlatTop(segment::SIZE / SQRT3);

impl SpaceShip {
    pub(crate) fn update(&mut self) {
        for (pos, segment) in self.grid.iter_mut() {
            let (x, y) = pos.to_pixel(SPACING);
            segment.update(self.pos + vec2(x, y));
        }
    }
    pub(crate) fn draw(&self) {
        for (pos, segment) in self.grid.iter() {
            let (x, y) = pos.to_pixel(SPACING);
            segment.draw(self.pos + vec2(x, y));
        }
    }
}

impl Gauge {
    fn draw_inner(&self, pos: Vec2, size: f32) {
        let start = *self.handle_range.start();
        let end = *self.handle_range.end();
        let range = end - start;
        let min = *self.value_range.start();
        let max = *self.value_range.end();
        draw_gauge_meter(pos, start, size, WHITE);
        draw_gauge_meter(pos, end, size, WHITE);
        let colors = [RED, GREEN, PINK, YELLOW];
        let mut prev = min;
        for (i, ((&val, &color), gh)) in self
            .data
            .iter()
            .zip(&colors)
            .zip(&self.data_sources)
            .enumerate()
        {
            let draw = |f: fn(Vec2, f32, f32, Color), val| {
                let percent = (val - min) / (max - min);
                let percent_angle = end - range * percent;
                f(pos, percent_angle, size * 0.75, color);
            };
            match gh.kind {
                GaugeHandleKind::Absolute => {
                    prev = val;
                    draw_circle(pos.x, pos.y, size / 5.0 - i as f32, color);
                    draw(draw_gauge_meter, val);
                }
                GaugeHandleKind::Relative => {
                    let mut steps = (val / (max - min) * 180.0).abs() as u32;
                    if val != 0.0 {
                        steps += 1;
                    }
                    for x in 0..steps {
                        draw(
                            draw_rel_gauge_meter,
                            prev + val.signum() * ((x + 1) as f32) * 5.0 / 180.0 * PI,
                        );
                    }
                    prev += val;
                }
            }
        }
        draw_circle_lines(pos.x, pos.y, size, 1.0, RED);
    }
}

fn draw_gauge_meter(pos: Vec2, angle: f32, size: f32, color: Color) {
    let vec = angle2vec(angle) * size;
    draw_line(pos.x, pos.y, pos.x + vec.x, pos.y + vec.y, 1.0, color);
}

fn draw_rel_gauge_meter(pos: Vec2, angle: f32, size: f32, color: Color) {
    let vec = angle2vec(angle) * size;
    let vec2 = vec * 0.75;
    draw_line(
        pos.x + vec2.x,
        pos.y + vec2.y,
        pos.x + vec.x,
        pos.y + vec.y,
        1.0,
        color,
    );
}
