use std::{collections::HashMap, convert::TryFrom as _, f64::consts::TAU};

use typed_floats::{NonNaNFinite, PositiveFinite};

use crate::OrbitKind;

struct Orbit {
    /// Angle of apehelion.
    angle: NonNaNFinite,
    /// Starting point of object in the orbit.
    t: PositiveFinite,
    /// raw orbit information.
    orbit: super::Orbit,
}

struct Object {
    orbit: Orbit,
}

#[derive(Default)]
pub struct Orbits {
    objects: HashMap<usize, Object>,
    next_id: usize,
}

pub struct ObjectId(#[expect(dead_code)] usize);

impl Orbits {
    pub fn insert(
        &mut self,
        angle: NonNaNFinite,
        orbit: super::Orbit,
        t: PositiveFinite,
    ) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(
            id,
            Object {
                orbit: Orbit { angle, orbit, t },
            },
        );
        ObjectId(id)
    }
    pub fn draw(
        &self,
        t: f64,
        segments: i32,
    ) -> impl Iterator<Item = (OrbitKind, (f32, f32), impl Iterator<Item = (f32, f32)> + '_)> + '_
    {
        self.objects.values().map(move |object| {
            let angle = object.orbit.orbit.angle_at(
                PositiveFinite::try_from(PositiveFinite::try_from(t).unwrap() + object.orbit.t)
                    .unwrap(),
            );
            let radius = object.orbit.orbit.r(angle);
            let system_angle = f64::from(angle + object.orbit.angle);
            let y = system_angle.sin();
            let x = system_angle.cos();
            let x = x as f32;
            let y = y as f32;
            let pos_x = x * f64::from(radius) as f32;
            let pos_y = y * f64::from(radius) as f32;

            let kind = object.orbit.orbit.kind();
            let (start, range) = match kind {
                OrbitKind::Circle | OrbitKind::Ellipse => (0.0, TAU),
                OrbitKind::Parabola | OrbitKind::Hyperbola => {
                    // 1/e = cos(angle)
                    let angle = (-1.0 / f64::from(object.orbit.orbit.epsilon)).acos();
                    let range = angle * 2.0;
                    // Subtract one degree so we don't render over infinity.
                    (-angle + TAU / 360.0, range - TAU / 180.0)
                }
            };
            let step_size = range / segments as f64;
            (
                kind,
                (pos_x, pos_y),
                (0..segments).map(move |i| {
                    let angle = step_size * (i + 1) as f64 + start;
                    let (new_y, new_x) = angle.sin_cos();
                    let mut new_x = new_x as f32;
                    let mut new_y = new_y as f32;
                    let r =
                        f64::from(object.orbit.orbit.r(
                            NonNaNFinite::try_from(angle - f64::from(object.orbit.angle)).unwrap(),
                        )) as f32;
                    new_y *= r;
                    new_x *= r;
                    (new_x, new_y)
                }),
            )
        })
    }
}
