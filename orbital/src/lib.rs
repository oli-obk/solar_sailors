use std::f32::consts::PI;

pub struct Orbit {
    /// Semi-latus rectum. Basically a factor scaling the height of the ellipse.
    pub p: f32,
    /// Eccentricity of the orbit. Basically means how wide it is.
    /// Must be in `0.0 ..= 1.0`
    pub epsilon: f32,
}

impl Orbit {
    pub fn circular(radius: f32) -> Self {
        Self {
            p: radius,
            epsilon: 0.0,
        }
    }

    /// Radius at orbital angle `phi` in orbit coordinates, not in the coordinate system of the center of gravity.
    pub fn r(&self, phi: f32) -> f32 {
        self.p / (1.0 + self.epsilon * phi.cos())
    }

    /// Radius at the point closest to the center of gravity.
    pub fn perihelion(&self) -> f32 {
        self.r(0.0)
    }

    /// Radius at the point farthest away from the center of gravity.
    pub fn aphelion(&self) -> f32 {
        self.r(PI)
    }

    /// Distance from center of ellipse to perihelion/aphelion
    pub fn semi_major(&self) -> f32 {
        self.p / (1.0 - self.epsilon * self.epsilon)
    }

    /// Distance from the center of the ellipse to the point at 90° to the semi major axis
    pub fn semi_minor(&self) -> f32 {
        self.p / (1.0 - self.epsilon * self.epsilon).sqrt()
    }

    pub fn area(&self) -> f32 {
        PI * self.semi_major() * self.semi_minor()
    }

    pub fn mean_motion(&self, central_mass: f32) -> f32 {
        let semi_major = self.semi_major();
        let specific_orbital_energy = -central_mass / (2.0 * semi_major);
        (-2.0 * specific_orbital_energy).sqrt() / semi_major
    }

    /// This cannot be solved numerically, we loop until the precision is
    /// in the 1e-6 range. Formula from https://space.stackexchange.com/questions/8911/determining-orbital-position-at-a-future-point-in-time
    pub fn eccentric_anomaly(&self, central_mass: f32, time: f32) -> f32 {
        let mean_anomaly = self.mean_motion(central_mass) * time;
        let mut e = mean_anomaly;
        loop {
            let delta =
                (e - self.epsilon * e.sin() - mean_anomaly) / (1.0 - self.epsilon * e.cos());
            e -= delta;
            if delta.abs() < 1e-6 {
                return e;
            }
        }
    }

    /// The angle of the object after `time` seconds, when starting at angle `0`
    pub fn angle_at(&self, central_mass: f32, time: f32) -> f32 {
        let e = self.eccentric_anomaly(central_mass, time);
        let x = e.cos() - self.epsilon;
        let y = e.sin() * (1.0 - self.epsilon * self.epsilon).sqrt();
        y.atan2(x)
    }
}
