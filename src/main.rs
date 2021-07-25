use macroquad::prelude::*;
use sail::Sail;
use ship::SpaceShip;
use stars::Stars;

mod controlled;
mod sail;
mod ship;
mod stars;

#[macroquad::main("BasicShapes")]
async fn main() {
    let stars = Stars::new(100);
    let mid_x = screen_width() / 2.0;
    let mid_y = screen_height() / 2.0;
    let sail = Sail::new(100.0, 100.0, 50.0, 10.0);
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
