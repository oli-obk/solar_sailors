use std::{collections::HashMap, convert::TryFrom as _, f64::consts::TAU};

use typed_floats::{NonNaN, NonNaNFinite, PositiveFinite};

use crate::{Orbit, OrbitKind};

pub struct Object {
    /// Angle of apehelion.
    pub angle: NonNaNFinite,
    /// Starting point of object in the orbit.
    pub t: PositiveFinite,
    /// raw orbit information.
    pub orbit: Orbit,
}

impl Object {
    pub fn angle_at(&self, t: f64) -> NonNaNFinite {
        self.orbit.angle_at(
            PositiveFinite::try_from(PositiveFinite::try_from(t).unwrap() + self.t).unwrap(),
        )
    }

    pub fn r(&self, angle: NonNaNFinite) -> NonNaN {
        self.orbit.r(angle)
    }
}

#[derive(Default)]
pub struct Orbits {
    sparse: HashMap<usize, usize>,
    next_id: usize,
    objects: Vec<Object>,
}

impl Orbits {
    /// Insert a new object. This operation is `O(1)`
    pub fn insert(&mut self, object: Object) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.sparse.insert(id, self.objects.len());
        self.objects.push(object);
        id
    }

    /// Remove an object. If it wasn't the last object to be removed,
    /// this operation may be expensive (`O(N)`).
    pub fn remove(&mut self, id: usize) -> Option<Object> {
        let idx = self.sparse.remove(&id)?;
        if self.objects.len() - 1 == idx {
            self.objects.pop()
        } else {
            for value in self.sparse.values_mut() {
                if *value > idx {
                    *value -= 1;
                }
            }
            Some(self.objects.remove(idx))
        }
    }

    /// Compute the position of all objects at time `t` and their corresponding orbits.
    /// The segments iterator is zero cost if unused.
    pub fn draw(
        &self,
        t: f64,
        segments: i32,
    ) -> impl Iterator<Item = (OrbitKind, (f32, f32), impl Iterator<Item = (f32, f32)> + '_)> + '_
    {
        self.objects.iter().map(move |object| {
            let angle = object.angle_at(t);
            let radius = object.r(angle);
            let system_angle = f64::from(angle + object.angle);
            let y = system_angle.sin();
            let x = system_angle.cos();
            let x = x as f32;
            let y = y as f32;
            let pos_x = x * f64::from(radius) as f32;
            let pos_y = y * f64::from(radius) as f32;

            let kind = object.orbit.kind();
            let mut step_size_start = None;
            (
                kind,
                (pos_x, pos_y),
                (0..segments).map(move |i| {
                    // FIXME: try out starting at the object position in case that is cheaper
                    let (step_size, start) = *step_size_start.get_or_insert_with(|| {
                        let (start, range) = match kind {
                            OrbitKind::Circle | OrbitKind::Ellipse => (0.0, TAU),
                            OrbitKind::Parabola | OrbitKind::Hyperbola => {
                                // 1/e = cos(angle)
                                let angle = (-1.0 / f64::from(object.orbit.epsilon)).acos();
                                let range = angle * 2.0;
                                // Subtract one degree so we don't render over infinity.
                                (-angle + TAU / 360.0, range - TAU / 180.0)
                            }
                        };
                        let step_size = range / segments as f64;
                        (step_size, start)
                    });
                    let angle = step_size * (i + 1) as f64 + start;
                    let (new_y, new_x) = angle.sin_cos();
                    let mut new_x = new_x as f32;
                    let mut new_y = new_y as f32;
                    let r = f64::from(
                        object
                            .orbit
                            .r(NonNaNFinite::try_from(angle - f64::from(object.angle)).unwrap()),
                    ) as f32;
                    new_y *= r;
                    new_x *= r;
                    (new_x, new_y)
                }),
            )
        })
    }
}
