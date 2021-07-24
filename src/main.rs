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
    loop {
        clear_background(BLACK);

        for &(x, y) in &stars {
            draw_star(Vec2::new(x, y), 5.0);
        }

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

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
