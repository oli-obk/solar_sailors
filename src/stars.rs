use macroquad::prelude::*;
use macroquad::rand::RandomRange;

#[derive(Default)]
pub struct Stars {
    positions: Vec<Vec2>,
    w: f32,
    h: f32,
}

pub fn rand_in_rect(rect: Rect) -> Vec2 {
    vec2(
        f32::gen_range(rect.left(), rect.right()),
        f32::gen_range(rect.top(), rect.bottom()),
    )
}

impl Stars {
    pub fn update(&mut self) {
        let w = screen_width();
        let h = screen_height();
        if w != self.w || h != self.h {
            self.w = w;
            self.h = h;
            let rect = Rect::new(0.0, 0.0, w, h);
            self.positions = (0..(w * h) as usize / 5000)
                .map(|_| rand_in_rect(rect))
                .collect();
        }
    }
    pub fn draw(&self) {
        push_camera_state();
        set_camera(&Camera2D::from_display_rect(Rect::new(
            0.0, 0.0, self.w, self.h,
        )));
        for &pos in &self.positions {
            draw_star(pos, 5.0);
        }
        pop_camera_state();
    }
}

fn draw_star(pos: Vec2, size: f32) {
    let y = f32::sin(std::f32::consts::PI / 3.0) * size;
    let x = size / 2.0;
    let left = Vec2::new(-x, y);
    let right = Vec2::new(x, y);
    draw_triangle(pos, pos + left, pos + right, WHITE)
}
