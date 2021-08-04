use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, PI};

use macroquad::prelude::*;

use crate::sail::{angle2vec, Sail};

pub(crate) struct SpaceShip {
    pub(crate) pos: Vec2,
    pub(crate) sail: Sail,
    pub(crate) width: f32,
    pub(crate) len: f32,
}

impl SpaceShip {
    pub(crate) fn update(&mut self) {
        self.sail.update();
    }
    pub(crate) fn draw(&self) {
        self.sail.draw();

        // Spaceship
        let mid = self.pos;
        draw_rectangle(mid.x - self.width / 2.0, mid.y, self.width, self.len, BLUE);

        let diff = self.sail.right_rope.value - self.sail.left_rope.value;
        // Gauges
        draw_gauge(
            mid + vec2(0.0, 30.0),
            20.0,
            [-self.sail.current_angle, diff / 10.0 + PI],
            -FRAC_PI_2,
            -FRAC_PI_2,
            FRAC_PI_2,
            FRAC_PI_2,
        );
        draw_gauge(
            mid + vec2(0.0, 70.0),
            20.0,
            [self.sail.force],
            0.0,
            -FRAC_PI_3 * 2.0,
            self.sail.sail_width.max,
            FRAC_PI_3 * 2.0,
        );
    }
}

fn draw_gauge<const N: usize>(
    pos: Vec2,
    size: f32,
    val: [f32; N],
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
    for (i, (val, color)) in val.iter().zip(colors).enumerate() {
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
