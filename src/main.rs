use macroquad::prelude::*;
use sail::Sail;
use ship::SpaceShip;
use stars::Stars;

mod controlled;
mod sail;
mod ship;
mod stars;

fn window_conf() -> Conf {
    Conf {
        window_title: "Solar Sailors".to_owned(),
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let stars = Stars::new(100, vec2(0.0, 0.0), vec2(400.0, 300.0));

    let sail = Sail::new(100.0, 100.0, 50.0, 10.0, vec2(0.0, 0.0));
    let mut ship = SpaceShip {
        sail,
        pos: Vec2::new(0.0, 0.0),
        root: Default::default(),
    };
    loop {
        let mut cam = Camera2D::default();
        let scale = (800.0 / screen_width()).max(600.0 / screen_height());
        cam.zoom.x = 1.0 / (scale * screen_width() / 2.0);
        cam.zoom.y = -1.0 / (scale * screen_height() / 2.0);
        set_camera(&cam);
        // Logic

        ship.update();

        // Drawing

        clear_background(BLACK);

        stars.draw();
        ship.draw();

        let pos = cam.screen_to_world(vec2(0.0, 0.0));
        draw_text(
            "SHIFT: pull/shrink any of the following",
            pos.x + 20.0,
            pos.y + 20.0,
            30.0,
            DARKGRAY,
        );
        draw_text("W: sail", pos.x + 20.0, pos.y + 40.0, 30.0, DARKGRAY);
        draw_text("S: left rope", pos.x + 20.0, pos.y + 60.0, 30.0, DARKGRAY);
        draw_text("D: right rope", pos.x + 20.0, pos.y + 80.0, 30.0, DARKGRAY);

        // Let the engine actually do stuff

        next_frame().await
    }
}
