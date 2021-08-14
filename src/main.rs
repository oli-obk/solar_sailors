use std::{
    cell::RefCell,
    collections::HashMap,
    f32::consts::{FRAC_PI_2, FRAC_PI_3, PI},
    rc::Rc,
};

use macroquad::prelude::*;
use sail::Sail;
use ship::{Gauge, Segment, SpaceShip};
use stars::Stars;

use crate::photons::PhotonMap;

mod controlled;
mod photons;
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
    let screen = Rect::new(-400.0, -300.0, 800.0, 600.0);
    let stars = Stars::new(100, screen);
    let mut photons = PhotonMap::new(100, screen);

    let sail_width = 50.0;
    let sail = Rc::new(RefCell::new(Sail::new(
        100.0,
        100.0,
        sail_width,
        10.0,
        vec2(0.0, 0.0),
    )));
    let sail_ref = Rc::downgrade(&sail);
    let sail_ref2 = sail_ref.clone();
    let sail_ref3 = sail_ref.clone();
    let mut ship = SpaceShip {
        sail,
        pos: Vec2::new(0.0, 0.0),
        grid: std::iter::repeat_with(HashMap::default).take(2).collect(),
    };
    ship.grid[0].insert(
        0,
        Segment {
            content: Some(Box::new(Gauge::new(
                vec![
                    Box::new(move || sail_ref.upgrade().map(|sail| sail.borrow().force))
                        as Box<dyn Fn() -> Option<f32>>,
                ],
                0.0..=sail_width,
                (-FRAC_PI_3 * 2.0)..=(FRAC_PI_3 * 2.0),
            ))),
            attachements: Default::default(),
        },
    );
    ship.grid[1].insert(
        0,
        Segment {
            content: Some(Box::new(Gauge::new(
                vec![
                    Box::new(move || sail_ref2.upgrade().map(|sail| -sail.borrow().current_angle))
                        as Box<dyn Fn() -> Option<f32>>,
                    Box::new(move || {
                        sail_ref3.upgrade().map(|sail| {
                            let sail = sail.borrow();
                            (sail.right_rope.value - sail.left_rope.value) / 10.0 + PI
                        })
                    }) as Box<dyn Fn() -> Option<f32>>,
                ],
                -FRAC_PI_2..=FRAC_PI_2,
                -FRAC_PI_2..=FRAC_PI_2,
            ))),
            attachements: Default::default(),
        },
    );
    loop {
        let mut cam = Camera2D::default();
        let scale = (800.0 / screen_width()).max(600.0 / screen_height());
        cam.zoom.x = 1.0 / (scale * screen_width() / 2.0);
        cam.zoom.y = -1.0 / (scale * screen_height() / 2.0);
        set_camera(&cam);

        // Logic
        ship.update();
        photons.update(ship.sail.borrow().rope_positions());

        // Drawing
        clear_background(BLACK);

        stars.draw();
        photons.draw();
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
