use macroquad::prelude::*;
use physics::Physics;
use rapier2d::prelude::*;
use sail::Sail;
use ship::SpaceShip;
use stars::Stars;

mod controlled;
mod physics;
mod sail;
mod ship;
mod stars;

#[macroquad::main("BasicShapes")]
async fn main() {
    let stars = Stars::new(100);
    let mid_x = screen_width() / 2.0;
    let mid_y = screen_height() / 2.0;

    let mut physics = Physics::new();

    let sail = Sail::new(&mut physics, 50.0, 10.0, vector![mid_x, mid_y]);
    let mut ship = SpaceShip {
        sail,
        len: 50.0,
        width: 20.0,
        pos: Vec2::new(mid_x, mid_y),
    };
    loop {
        // Logic

        ship.update(&mut physics);

        physics.update();

        // Drawing

        clear_background(BLACK);

        stars.draw();
        ship.draw(&physics);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        // Let the engine actually do stuff

        next_frame().await
    }
}
