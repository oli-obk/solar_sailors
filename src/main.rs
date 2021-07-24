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
    let mid_x = screen_width() / 2.0;
    let mid_y = screen_height() / 2.0;
    let sail = Sail::new(Vec2::new(mid_x, mid_y), 100.0, 100.0, 50.0);
    let mut ship = SpaceShip {
        sail,
        len: 50.0,
        width: 20.0,
    };
    loop {
        clear_background(BLACK);

        for &(x, y) in &stars {
            draw_star(Vec2::new(x, y), 5.0);
        }

        ship.update();
        ship.draw();

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

struct SpaceShip {
    sail: Sail,
    width: f32,
    len: f32,
}

impl SpaceShip {
    fn update(&mut self) {
        self.sail.update();
    }
    fn draw(&self) {
        self.sail.draw();

        // Spaceship
        let mid = self.sail.anchor;
        draw_triangle(
            mid,
            mid + Vec2::new(10.0, self.width / 2.0),
            mid + Vec2::new(-10.0, self.width / 2.0),
            BLUE,
        );
        draw_rectangle(mid.x - self.width / 2.0, mid.y + 10.0, self.width, self.len, BLUE);
    }
}

struct Sail {
    anchor: Vec2,
    left_rope: ButtonControlledRange,
    right_rope: ButtonControlledRange,
    sail_width: ButtonControlledRange,
}

impl Sail {
    fn new(anchor: Vec2, left_rope: f32, right_rope: f32, sail_width: f32) -> Self {
        Self {
            anchor,
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
    fn draw(&self) {
        let (left_x, left_y, right_x, right_y) = self.rope_positions();
        // Sail
        draw_line(left_x, left_y, right_x, right_y, 1.0, YELLOW);
        // Ropes
        draw_line(self.anchor.x, self.anchor.y, left_x, left_y, 1.0, GRAY);
        draw_line(self.anchor.x, self.anchor.y, right_x, right_y, 1.0, GRAY);
    }

    /// Compute the position of the sail corners
    fn rope_positions(&self) -> (f32, f32, f32, f32) {
        let angle = rope_angle(
            self.left_rope.value,
            self.right_rope.value,
            self.sail_width.value,
        );
        let half_angle = angle / 2.0;
        let x = half_angle.sin();
        let y = half_angle.cos();
        (
            -x * self.left_rope.value + self.anchor.x,
            -y * self.left_rope.value + self.anchor.y,
            x * self.right_rope.value + self.anchor.x,
            -y * self.right_rope.value + self.anchor.y,
        )
    }
}

struct ButtonControlledRange {
    value: f32,
    keycode: KeyCode,
}

impl ButtonControlledRange {
    fn new(start: f32, keycode: KeyCode) -> Self {
        Self {
            value: start,
            keycode,
        }
    }
    fn update(&mut self) {
        if is_key_down(self.keycode) {
            if is_key_down(KeyCode::LeftShift) {
                self.value -= 1.0;
            } else {
                self.value += 1.0;
            }
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
