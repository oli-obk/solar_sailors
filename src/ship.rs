use std::collections::HashMap;

use hex2d::Spacing;
use macroquad::prelude::*;

mod segment;
mod segments {
    mod gauge;
    pub use gauge::Gauge;
}

mod attachements {
    mod sail;
    pub use sail::*;
}

pub use segment::*;
pub use segments::*;
pub use attachements::*;

pub(crate) struct SpaceShip {
    pub(crate) pos: Vec2,
    pub(crate) grid: HashMap<hex2d::Coordinate, Segment>,
}

// sqrt is not const fn, so we inline sqrt(3)
pub const SQRT3: f32 = 1.73205080757;
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

fn draw_gauge(
    pos: Vec2,
    size: f32,
    val: impl IntoIterator<Item = f32>,
    min: f32,
    min_handle_pos: f32,
    max: f32,
    max_handle_pos: f32,
) {
    let start = min_handle_pos;
    let end = max_handle_pos;
    let range = max_handle_pos - min_handle_pos;
    draw_gauge_meter(pos, start, size, WHITE);
    draw_gauge_meter(pos, end, size, WHITE);
    let colors = [RED, GREEN, PINK, YELLOW];
    for (i, (val, &color)) in val.into_iter().zip(&colors).enumerate() {
        let percent = (val - min) / (max - min);
        let percent_angle = end - range * percent;
        draw_gauge_meter(pos, percent_angle, size * 0.75, color);
        draw_circle(pos.x, pos.y, size / 5.0 - i as f32, color);
    }
    draw_circle_lines(pos.x, pos.y, size, 1.0, RED);
}

fn draw_gauge_meter(pos: Vec2, angle: f32, size: f32, color: Color) {
    let vec = angle2vec(angle) * size;
    draw_line(pos.x, pos.y, pos.x + vec.x, pos.y + vec.y, 1.0, color);
}
