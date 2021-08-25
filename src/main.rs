use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI},
};

use macroquad::prelude::*;
use sail::Sail;
use ship::{Gauge, Segment, SpaceShip};
use stars::Stars;

use crate::{photons::PhotonMap, player::Player, ship::Attachement};

mod controlled;
mod datastructures;
mod orbits;
mod photons;
mod player;
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
    let mut player = Player::new((0, -1), 3);
    let screen = Rect::new(-400.0, -300.0, 800.0, 600.0);
    let stars = Stars::new(100, screen);
    let mut orbits = orbits::Orbits::new();
    orbits.insert(FRAC_PI_4, orbital::Orbit::circular(200.0));
    orbits.insert(
        0.0,
        orbital::Orbit {
            p: 50.0,
            epsilon: 0.8,
        },
    );
    let mut photons = PhotonMap::new(100, screen);

    let sail_width = 100.0;
    let (sail, rope_positions, force, current_angle, left_rope, right_rope) =
        Sail::new(200.0, 200.0, sail_width, 20.0, -FRAC_PI_3);
    photons.sails.push(rope_positions);
    let mut ship = SpaceShip {
        pos: Vec2::new(0.0, 0.0),
        grid: HashMap::default(),
    };
    let mut attachements: [Option<Box<dyn Attachement>>; 6] = Default::default();
    attachements[1] = Some(Box::new(sail));
    ship.grid.insert(
        (0, 0).into(),
        Segment {
            content: Some(Box::new(Gauge::new(
                vec![Box::new(move || force.get()) as _],
                0.0..=sail_width,
                (-FRAC_PI_3 * 2.0)..=(FRAC_PI_3 * 2.0),
            ))),
            attachements,
        },
    );
    ship.grid.insert(
        (0, -1).into(),
        Segment {
            content: Some(Box::new(Gauge::new(
                vec![
                    Box::new(move || current_angle.get().map(|a| -a)) as _,
                    Box::new(move || Some((right_rope.get()? - left_rope.get()?) / 10.0 + PI)) as _,
                ],
                -FRAC_PI_2..=FRAC_PI_2,
                -FRAC_PI_2..=FRAC_PI_2,
            ))),
            attachements: Default::default(),
        },
    );

    let mut window = GameWindow::Ship;
    loop {
        // Logic
        ship.update();
        photons.update();
        orbits.update();
        player.update(&mut ship.grid);

        if is_key_pressed(KeyCode::M) {
            window = match window {
                GameWindow::Ship => GameWindow::Orbit,
                GameWindow::Orbit => GameWindow::Ship,
            };
        }

        let mut cam = Camera2D::default();
        let scale = (800.0 / screen_width()).max(600.0 / screen_height());
        cam.zoom.x = 1.0 / (scale * screen_width() / 2.0);
        cam.zoom.y = -1.0 / (scale * screen_height() / 2.0);
        set_camera(&cam);

        // Drawing
        clear_background(BLACK);

        match window {
            GameWindow::Ship => {
                stars.draw();
                photons.draw();
                ship.draw();
                player.draw();

                let pos = cam.screen_to_world(vec2(0.0, 0.0));
                draw_text("M: view orbit", pos.x + 20.0, pos.y + 20.0, 30.0, DARKGRAY);
                draw_text(
                    "WASD: control crab, up/down acts on things next to crab",
                    pos.x + 20.0,
                    pos.y + 40.0,
                    30.0,
                    DARKGRAY,
                );
            }
            GameWindow::Orbit => {
                orbits.draw();
                let pos = cam.screen_to_world(vec2(0.0, 0.0));
                draw_text(
                    "M: return to ship",
                    pos.x + 20.0,
                    pos.y + 20.0,
                    30.0,
                    DARKGRAY,
                );
            }
        }

        // Let the engine actually do stuff

        next_frame().await
    }
}

enum GameWindow {
    Ship,
    Orbit,
}
