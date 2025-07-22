use std::convert::TryInto as _;

use ::orbits::*;
use macroquad::{miniquad::window::screen_size, prelude::*};

#[macroquad::main("solar playground")]
async fn main() {
    prevent_quit();

    let mut orbits = orbits::Orbits::default();
    for i in 0..10 {
        orbits.insert(Orbit::from_pos_dir(
            100.0.try_into().unwrap(),
            1.0.try_into().unwrap(),
            (i as f64 / 100.).try_into().unwrap(),
            0.1.try_into().unwrap(),
        ));
    }
    let mut mouse_down = None;
    let mut last_orbit = None;

    while !is_quit_requested() || is_key_pressed(KeyCode::Escape) {
        let s = Vec2::from(screen_size()) / 2.;
        draw_circle(s.x, s.y, 50., YELLOW);

        if is_mouse_button_down(MouseButton::Left) {
            let pos = Vec2::from(mouse_position());
            let mouse_down = *mouse_down.get_or_insert(pos);
            draw_circle(mouse_down.x, mouse_down.y, 10., RED);
            if pos.distance_squared(mouse_down) > 50. {
                if let Some(last_orbit) = last_orbit.take() {
                    orbits.remove(last_orbit);
                }
                let d = (pos - mouse_down).as_dvec2() / 1000.;
                let mouse_down = (mouse_down - s).as_dvec2();
                last_orbit = Some(orbits.insert(Orbit::from_pos_dir(
                    mouse_down.x.try_into().unwrap(),
                    mouse_down.y.try_into().unwrap(),
                    d.x.try_into().unwrap(),
                    d.y.try_into().unwrap(),
                )));
            }
        } else {
            mouse_down = None;
            last_orbit = None;
        }

        for (_, (_x, _y), mut points) in orbits.draw(0.0, 300) {
            let start = points.next().unwrap();
            let (mut x, mut y) = start;

            for (new_x, new_y) in points {
                draw_line(x + s.x, y + s.y, new_x + s.x, new_y + s.y, 1., WHITE);
                x = new_x;
                y = new_y;
            }
            draw_line(start.0 + s.x, start.1 + s.y, s.x + x, s.y + y, 1., WHITE);
        }

        next_frame().await;
    }
}
