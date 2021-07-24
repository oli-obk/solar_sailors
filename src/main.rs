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
    let mut left_rope = 50.0;
    let mut right_rope = 50.0;
    let sail_width = 100.0;
    let mid_x = screen_width() / 2.0;
    let mid_y = screen_height() / 2.0;
    loop {
        clear_background(BLACK);

        for &(x, y) in &stars {
            draw_star(Vec2::new(x, y), 5.0);
        }

        if is_key_down(KeyCode::W) {
            left_rope += 1.0;
            right_rope += 1.0;
        } else if is_key_down(KeyCode::S) {
            left_rope -= 1.0;
            right_rope -= 1.0;
        } else if is_key_down(KeyCode::A) {
            left_rope += 1.0;
        } else if is_key_down(KeyCode::D) {
            right_rope += 1.0;
        }

        let left_x = mid_x - sail_width / 2.0;
        let left_y = mid_y - left_rope;
        let right_x = mid_x + sail_width / 2.0;
        let right_y = mid_y - right_rope;
        // Sail
        draw_line(left_x, left_y, right_x, right_y, 1.0, YELLOW);
        // Ropes
        draw_line(mid_x, mid_y, left_x, left_y, 1.0, GRAY);
        draw_line(mid_x, mid_y, right_x, right_y, 1.0, GRAY);

        // Spaceship
        draw_circle(mid_x, mid_y, 20.0, BLUE);

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
