use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI},
};

use macroquad::prelude::*;
use ship::{Gauge, Segment, SpaceShip};
use stars::Stars;

use crate::{
    player::Player,
    save::*,
    ship::{Attachement, Map, Sail},
};

mod controlled;
mod datastructures;
mod orbits;
mod player;
mod save;
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
    let mut player = Player::new(
        (
            load("player/x").unwrap_or(0),
            load("player/y").unwrap_or(-1),
        ),
        3,
    );
    let mut stars = Stars::default();
    let orbit_render_target = render_target(1024, 1024);
    let map = Map {
        texture: orbit_render_target.texture,
        zoom: 0.1,
        small_zoom: 1.0,
    };
    let mut orbits = orbits::Orbits::load();
    orbits.insert(FRAC_PI_4, orbital::Orbit::circular(200.0));
    orbits.insert(
        0.0,
        orbital::Orbit {
            p: 50.0,
            epsilon: 0.8,
        },
    );

    let sail_width = 100.0;
    let (sail, rope_positions, force, current_angle, left_rope, right_rope) =
        Sail::new(200.0, 200.0, sail_width, 20.0, -FRAC_PI_3);
    stars.sails.push(rope_positions);
    let mut ship = SpaceShip {
        pos: Vec2::new(0.0, 0.0),
        grid: HashMap::default(),
    };
    let mut attachements: [Option<Box<dyn Attachement>>; 6] = Default::default();
    attachements[1] = Some(Box::new(sail));
    attachements[4] = Some(Box::new(map));
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
        stars.update();
        ship.update();
        orbits.update();
        player.update(&mut ship.grid);

        if is_key_pressed(KeyCode::M) {
            window = match window {
                GameWindow::Ship => GameWindow::Orbit,
                GameWindow::Orbit => GameWindow::Ship,
            };
        }

        let mut cam = Camera2D::default();
        cam.zoom /= 200.0;
        cam.render_target = Some(orbit_render_target);
        set_camera(&cam);
        clear_background(Color::default());
        orbits.draw();

        let mut cam = Camera2D::default();
        cam.zoom.x = 1.0 / (screen_width() / 2.0);
        cam.zoom.y = -1.0 / (screen_height() / 2.0);
        set_camera(&cam);

        // Drawing
        clear_background(BLACK);

        stars.draw();
        ship.draw();
        player.draw();

        let pos = cam.screen_to_world(vec2(0.0, 0.0));
        draw_text(
            "WASD: control crab, up/down acts on things next to crab",
            pos.x + 20.0,
            pos.y + 20.0,
            30.0,
            DARKGRAY,
        );

        // Let the engine actually do stuff

        next_frame().await
    }
}

enum GameWindow {
    Ship,
    Orbit,
}
