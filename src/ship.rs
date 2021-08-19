use std::collections::HashMap;

use macroquad::prelude::*;

mod segment;
mod segments {
    mod gauge;
    pub use gauge::Gauge;
}

use crate::sail::angle2vec;
pub use segment::*;
pub use segments::*;

pub(crate) struct SpaceShip {
    pub(crate) pos: Vec2,
    pub(crate) grid: Vec<HashMap<isize, Segment>>,
}

impl SpaceShip {
    pub(crate) fn update(&mut self) {
        for row in &mut self.grid {
            for segment in row.values_mut() {
                segment.update();
            }
        }
    }
    pub(crate) fn draw(&self) {
        let height = segment::SIZE;
        for (level, row) in self.grid.iter().enumerate() {
            let y = level as f32 * height + height / 2.0;
            for (&pos, segment) in row {
                let y = (pos % 2) as f32 * height + y;
                let x = segment::SIZE / (3.0_f32).sqrt() * pos as f32;
                segment.draw(self.pos + vec2(x, y));
            }
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
