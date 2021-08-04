use macroquad::prelude::*;
use macroquad::rand::RandomRange;

pub struct Stars {
    positions: Vec<Vec2>,
}

impl Stars {
    pub fn new(count: usize, pos: Vec2, size: Vec2) -> Self {
        Self {
            positions: (0..count)
                .map(|_| {
                    Vec2::new(
                        f32::gen_range(pos.x - size.x, pos.x + size.x),
                        f32::gen_range(pos.y - size.y, pos.y + size.y),
                    )
                })
                .collect(),
        }
    }
    pub fn draw(&self) {
        for &pos in &self.positions {
            draw_star(pos, 5.0);
        }
    }
}

fn draw_star(pos: Vec2, size: f32) {
    let y = f32::sin(std::f32::consts::PI / 3.0) * size;
    let x = size / 2.0;
    let left = Vec2::new(-x, y);
    let right = Vec2::new(x, y);
    draw_triangle(pos, pos + left, pos + right, WHITE)
}
