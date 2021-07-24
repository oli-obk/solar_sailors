use macroquad::prelude::*;
use macroquad::rand::RandomRange;

#[macroquad::main("BasicShapes")]
async fn main() {
    let stars: Vec<_> = (0..100)
        .map(|_| {
            (
                f32::gen_range(0.0, screen_width()),
                f32::gen_range(0.0, screen_height()),
            )
        })
        .collect();
    let mut left_rope = 100.0;
    let mut right_rope = 100.0;
    let mut sail_width = 50.0;
    let mid_x = screen_width() / 2.0;
    let mid_y = screen_height() / 2.0;
    loop {
        clear_background(BLACK);

        for &(x, y) in &stars {
            draw_star(Vec2::new(x, y), 5.0);
        }

        if is_key_down(KeyCode::W) {
            if is_key_down(KeyCode::LeftShift) {
                sail_width -= 1.0;
            } else {
                sail_width += 1.0;
            }
        }
        if is_key_down(KeyCode::A) {
            if is_key_down(KeyCode::LeftShift) {
                left_rope -= 1.0;
            } else {
                left_rope += 1.0;
            }
        }
        if is_key_down(KeyCode::D) {
            if is_key_down(KeyCode::LeftShift) {
                right_rope -= 1.0;
            } else {
                right_rope += 1.0;
            }
        }

        let (left_x, left_y, right_x, right_y) = rope_positions(left_rope, right_rope, sail_width);
        // Sail
        draw_line(
            mid_x + left_x,
            mid_y + left_y,
            mid_x + right_x,
            mid_y + right_y,
            1.0,
            YELLOW,
        );
        // Ropes
        draw_line(mid_x, mid_y, mid_x + left_x, mid_y + left_y, 1.0, GRAY);
        draw_line(mid_x, mid_y, mid_x + right_x, mid_y + right_y, 1.0, GRAY);

        // Spaceship
        let mid = Vec2::new(mid_x, mid_y);
        let len = 50.0;
        let width = 20.0;
        draw_triangle(
            mid,
            mid + Vec2::new(10.0, width / 2.0),
            mid + Vec2::new(-10.0, width / 2.0),
            BLUE,
        );
        draw_rectangle(mid_x - width / 2.0, mid_y + 10.0, width, len, BLUE);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn draw_star(pos: Vec2, size: f32) {
    let y = f32::sin(std::f32::consts::PI / 3.0) * size;
    let x = size / 2.0;
    let left = Vec2::new(-x, y);
    let right = Vec2::new(x, y);
    draw_triangle(pos, pos + left, pos + right, WHITE)
}

/// Compute the position of the sail corners
fn rope_positions(left: f32, right: f32, sail: f32) -> (f32, f32, f32, f32) {
    let angle = rope_angle(left, right, sail);
    let half_angle = angle / 2.0;
    let x = half_angle.sin();
    let y = half_angle.cos();
    (-x * left, -y * left, x * right, -y * right)
}

/// Find the angle between "a" and "b" for a triangle with the given three side lengths.
fn rope_angle(a: f32, b: f32, c: f32) -> f32 {
    // http://mathcentral.uregina.ca/QQ/database/QQ.09.07/h/lucy1.html
    // c^2 = a^2 + b^2 - 2ab cos(C)
    // 2ab cos(C) = a^2 + b^2 - c^2
    // cos(C) = (a^2 + b^2 - c^2)/(2ab)
    let squares = a * a + b * b - c * c;
    let divisor = 2.0 * a * b;
    let cos_c = squares / divisor;
    cos_c.acos()
}
