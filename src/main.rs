use controlled::ButtonControlledRange;
use macroquad::prelude::*;
use macroquad::rand::RandomRange;

mod controlled;

#[macroquad::main("BasicShapes")]
async fn main() {
    let stars = Stars::new(100);
    let mid_x = screen_width() / 2.0;
    let mid_y = screen_height() / 2.0;
    let sail = Sail::new(100.0, 100.0, 50.0);
    let mut ship = SpaceShip {
        sail,
        len: 50.0,
        width: 20.0,
        pos: Vec2::new(mid_x, mid_y),
    };
    loop {
        clear_background(BLACK);

        ship.update();

        stars.draw();
        ship.draw();

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

struct Stars {
    positions: Vec<Vec2>,
}

impl Stars {
    fn new(count: usize) -> Self {
        Self {
            positions: (0..count)
                .map(|_| {
                    Vec2::new(
                        f32::gen_range(0.0, screen_width()),
                        f32::gen_range(0.0, screen_height()),
                    )
                })
                .collect(),
        }
    }
    fn draw(&self) {
        for &pos in &self.positions {
            draw_star(pos, 5.0);
        }
    }
}

struct SpaceShip {
    pos: Vec2,
    sail: Sail,
    width: f32,
    len: f32,
}

impl SpaceShip {
    fn update(&mut self) {
        self.sail.update();
    }
    fn draw(&self) {
        self.sail.draw(self.pos);

        // Spaceship
        let mid = self.pos;
        draw_triangle(
            mid,
            mid + Vec2::new(10.0, self.width / 2.0),
            mid + Vec2::new(-10.0, self.width / 2.0),
            BLUE,
        );
        draw_rectangle(
            mid.x - self.width / 2.0,
            mid.y + 10.0,
            self.width,
            self.len,
            BLUE,
        );
    }
}

struct Sail {
    left_rope: ButtonControlledRange,
    right_rope: ButtonControlledRange,
    sail_width: ButtonControlledRange,
}

impl Sail {
    fn new(left_rope: f32, right_rope: f32, sail_width: f32) -> Self {
        Self {
            left_rope: ButtonControlledRange::new(left_rope, KeyCode::A),
            right_rope: ButtonControlledRange::new(right_rope, KeyCode::D),
            sail_width: ButtonControlledRange::new(sail_width, KeyCode::W),
        }
    }
    fn update(&mut self) {
        self.left_rope.update();
        self.right_rope.update();
        self.sail_width.update();
    }
    fn draw(&self, anchor: Vec2) {
        let (left_x, left_y, right_x, right_y) = self.rope_positions(anchor);
        // Sail
        draw_line(left_x, left_y, right_x, right_y, 1.0, YELLOW);
        // Ropes
        draw_line(anchor.x, anchor.y, left_x, left_y, 1.0, GRAY);
        draw_line(anchor.x, anchor.y, right_x, right_y, 1.0, GRAY);
    }

    /// Compute the position of the sail corners
    fn rope_positions(&self, anchor: Vec2) -> (f32, f32, f32, f32) {
        let angle = rope_angle(
            self.left_rope.value,
            self.right_rope.value,
            self.sail_width.value,
        );
        let half_angle = angle / 2.0;
        let x = half_angle.sin();
        let y = half_angle.cos();
        (
            -x * self.left_rope.value + anchor.x,
            -y * self.left_rope.value + anchor.y,
            x * self.right_rope.value + anchor.x,
            -y * self.right_rope.value + anchor.y,
        )
    }
}

fn draw_star(pos: Vec2, size: f32) {
    let y = f32::sin(std::f32::consts::PI / 3.0) * size;
    let x = size / 2.0;
    let left = Vec2::new(-x, y);
    let right = Vec2::new(x, y);
    draw_triangle(pos, pos + left, pos + right, WHITE)
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
