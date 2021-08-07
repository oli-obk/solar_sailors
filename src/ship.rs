use std::{
    cell::RefCell,
    f32::consts::{FRAC_PI_2, PI},
    rc::Rc,
};

use macroquad::prelude::*;

mod segment;
mod segments {
    mod gauge;
    pub use gauge::Gauge;
}

use crate::sail::{angle2vec, Sail};
pub use segment::*;
pub use segments::*;

pub(crate) struct SpaceShip {
    pub(crate) pos: Vec2,
    pub(crate) sail: Rc<RefCell<Sail>>,
    pub(crate) root: Rc<SharedElement<Segment>>,
}

impl SpaceShip {
    pub(crate) fn update(&mut self) {
        self.sail.borrow_mut().update();
        self.root.update();
    }
    pub(crate) fn draw(&self) {
        self.root.draw(self.pos + vec2(0.0, 20.0));
        self.sail.borrow().draw();
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
    for (i, (val, color)) in val.into_iter().zip(colors).enumerate() {
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
