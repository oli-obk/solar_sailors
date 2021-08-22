use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI},
};

use macroquad::prelude::{
    animation::{AnimatedSprite, Animation, AnimationFrame},
    *,
};
use sail::Sail;
use ship::{Gauge, Segment, SpaceShip};
use stars::Stars;

use crate::{photons::PhotonMap, ship::Attachement};

mod controlled;
mod datastructures;
mod orbits;
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
    let crab = Texture2D::from_file_with_format(
        include_bytes!("../assets/redCrab.png"),
        Some(ImageFormat::Png),
    );
    // Pixel art
    crab.set_filter(FilterMode::Nearest);
    let mut crabi = 0;
    let mut crabx = 0.0;
    let animations = &[
        Animation {
            name: "dance".into(),
            row: 0,
            frames: 7,
            fps: 7,
        },
        Animation {
            name: "walk left".into(),
            row: 1,
            frames: 4,
            fps: 7,
        },
        Animation {
            name: "walk right".into(),
            row: 2,
            frames: 4,
            fps: 7,
        },
        Animation {
            name: "idle".into(),
            row: 3,
            frames: 6,
            fps: 7,
        },
        Animation {
            name: "attack".into(),
            row: 4,
            frames: 7,
            fps: 7,
        },
        Animation {
            name: "sit".into(),
            row: 5,
            frames: 4,
            fps: 7,
        },
        Animation {
            name: "jump".into(),
            row: 4,
            frames: 3,
            fps: 7,
        },
    ];
    let mut anim = AnimatedSprite::new(16, 16, animations, true);
    anim.set_animation(1);

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

    let sail_width = 50.0;
    let (sail, rope_positions, force, current_angle, left_rope, right_rope) =
        Sail::new(100.0, 100.0, sail_width, 10.0, -FRAC_PI_3);
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
                vec![Box::new(move || force.get()) as Box<dyn Fn() -> Option<f32>>],
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
                    Box::new(move || current_angle.get().map(|a| -a))
                        as Box<dyn Fn() -> Option<f32>>,
                    Box::new(move || Some((right_rope.get()? - left_rope.get()?) / 10.0 + PI))
                        as Box<dyn Fn() -> Option<f32>>,
                ],
                -FRAC_PI_2..=FRAC_PI_2,
                -FRAC_PI_2..=FRAC_PI_2,
            ))),
            attachements: Default::default(),
        },
    );

    let mut window = GameWindow::Ship;
    let mut speed = 0;
    loop {
        // Logic
        ship.update();
        photons.update();
        orbits.update();
        anim.update();
        speed += 1;
        if speed == 8 {
            speed = 0;
            crabi += 1;
            crabi %= 4;
            if crabi == 3 {
                crabx += 3.0;
            }
        }
        anim.set_frame(crabi);

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
                let AnimationFrame {
                    source_rect,
                    dest_size,
                } = anim.frame();
                draw_texture_ex(
                    crab,
                    crabx,
                    264.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(dest_size * 3.0),
                        source: Some(source_rect),
                        ..Default::default()
                    },
                );

                let pos = cam.screen_to_world(vec2(0.0, 0.0));
                draw_text("M: view orbit", pos.x + 20.0, pos.y + 20.0, 30.0, DARKGRAY);
                draw_text(
                    "hold SHIFT with any of the following for inverse effect",
                    pos.x + 20.0,
                    pos.y + 40.0,
                    30.0,
                    DARKGRAY,
                );
                draw_text("W: expand sail", pos.x + 20.0, pos.y + 60.0, 30.0, DARKGRAY);
                draw_text(
                    "A: let out left rope",
                    pos.x + 20.0,
                    pos.y + 80.0,
                    30.0,
                    DARKGRAY,
                );
                draw_text(
                    "D: let out right rope",
                    pos.x + 20.0,
                    pos.y + 100.0,
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
