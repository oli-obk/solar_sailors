use std::{
    collections::HashMap,
    convert::{TryFrom as _, TryInto as _},
    f32::consts::{FRAC_PI_2, FRAC_PI_3, PI},
    sync::{Arc, Mutex},
};

use macroquad::prelude::{
    coroutines::{start_coroutine, wait_seconds},
    *,
};
use save::Saveable;
use ship::{Gauge, GaugeHandle, Segment, SpaceShip};
use stars::Stars;

use crate::{
    datastructures::SetGet,
    player::Player,
    ship::{Attachement, Map, Sail},
};

mod controlled;
mod datastructures;
mod orbits;
mod player;
mod save;
mod ship;
mod stars;

use orbital::typed_floats::{NonNaNFinite, StrictlyPositiveFinite};

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
    let mut stars = Stars::default();
    let orbit_render_target = render_target(1024, 1024);
    let map = Map {
        texture: orbit_render_target.texture.clone(),
        zoom: Saveable::new(0.5, "map_zoom"),
        small_zoom: 1.0,
    };
    let mut orbits = orbits::Orbits::load();
    orbits.insert(
        NonNaNFinite::<f64>::try_from(0.0).unwrap(),
        orbital::Orbit::circular(200.0_f64.try_into().unwrap()),
        0.0.try_into().unwrap(),
    );
    // Find a parabolic orbit
    let mut dy = 0.141.try_into().unwrap();
    loop {
        let (orbit, angle, t) = orbital::Orbit::from_pos_dir(
            100.0.try_into().unwrap(),
            0.0.try_into().unwrap(),
            0.0.try_into().unwrap(),
            dy,
        );
        let diff = StrictlyPositiveFinite::<f64>::try_from(1.0).unwrap() - orbit.epsilon;
        let diff = diff * StrictlyPositiveFinite::<f64>::try_from(0.01).unwrap();
        if diff.abs() < 1e-6 {
            orbits.insert(angle, orbit, t);
            break;
        }
        dy += NonNaNFinite::try_from(diff).unwrap();
    }

    let sail_width = 100.0;
    let (
        sail,
        ship::SailParameters {
            rope_positions,
            force,
            current_angle,
            left_rope,
            right_rope,
        },
    ) = Sail::new(200.0, 200.0, sail_width, 20.0, -FRAC_PI_3);
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
                [
                    GaugeHandle::from(move |_| force.get()),
                    GaugeHandle::from(move |diff| Some(diff * 100.0_f32)).relative(),
                ],
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
                [
                    GaugeHandle::from(move |_| Some(-current_angle.get()?)),
                    GaugeHandle::from(move |diff| Some(diff * 1000.0_f32)).relative(),
                    GaugeHandle::from(move |_| {
                        Some((right_rope.get()? - left_rope.get()?) / 10.0 + PI)
                    }),
                ],
                -FRAC_PI_2..=FRAC_PI_2,
                -FRAC_PI_2..=FRAC_PI_2,
            ))),
            attachements: Default::default(),
        },
    );

    let mut window = GameWindow::Ship;
    let mut delete_save = start_coroutine(async {});
    let do_delete = Arc::new(Mutex::new(false));
    save::transaction_loop(|| {
        if delete_save.is_done() {
            if is_key_down(KeyCode::Delete) {
                let do_delete = do_delete.clone();
                delete_save = start_coroutine(async move {
                    let timer = start_coroutine(async {
                        wait_seconds(2.0).await;
                    });
                    while is_key_down(KeyCode::Delete) {
                        if timer.is_done() {
                            *do_delete.lock().unwrap() = true;
                            return;
                        }
                        next_frame().await
                    }
                })
            }
            if *do_delete.lock().unwrap() {
                orbits.t.set(-1000.0);
                *do_delete.lock().unwrap() = false;
            }
        }
        // Logic
        if !(cfg!(debug_assertions) && is_key_down(KeyCode::Space)) {
            stars.update();
            ship.update();
            orbits.update();
            player.update(&mut ship.grid);
        }

        if is_key_pressed(KeyCode::M) {
            window = match window {
                GameWindow::Ship => GameWindow::Orbit,
                GameWindow::Orbit => GameWindow::Ship,
            };
        }

        let mut cam = Camera2D::default();
        cam.zoom /= 300.0;
        cam.render_target = Some(orbit_render_target.clone());
        set_camera(&cam);
        clear_background(Color::default());
        orbits.draw();

        // HACK: remove after https://github.com/not-fl3/macroquad/pull/824 makes it into a release
        set_default_camera();

        let mut cam = Camera2D::default();
        cam.zoom.x = 1.0 / (screen_width() / 2.0);
        cam.zoom.y = 1.0 / (screen_height() / 2.0);
        set_camera(&cam);

        // Drawing
        clear_background(BLACK);

        stars.draw();
        ship.draw();
        player.draw();

        let pos = cam.screen_to_world(vec2(0.0, 0.0));
        draw_text(
            "A: left, D: right, W/S control element at location",
            pos.x + 20.0,
            pos.y + 20.0,
            30.0,
            DARKGRAY,
        );

        // Let the engine actually do stuff

        next_frame()
    })
    .await;
}

enum GameWindow {
    Ship,
    Orbit,
}
