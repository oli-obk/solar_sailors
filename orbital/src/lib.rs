use std::f32::consts::{PI, TAU};

#[derive(Debug)]
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

    /// Compute orbit from position and speed. The second return value is the angle of the orbit.
    pub fn from_pos_dir(x: f32, y: f32, dx: f32, dy: f32) -> (Self, f32) {
        let r_squared = x * x + y * y;
        let r = r_squared.sqrt();
        let phi = y.atan2(x);
        let v_squared = dx * dx + dy * dy;
        let a = r / (2.0 - r * v_squared);
        let xi = dy.atan2(dx);
        let sinxiphi = (xi - phi).sin();
        let sinxiphi2 = sinxiphi * sinxiphi;
        // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Book%3A_Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector
        // formula 9.9.4
        let e_squared = 1.0 - r_squared * v_squared * sinxiphi2 / a;
        let e = e_squared.sqrt();
        let cos_angle = (a * (1.0 - e_squared) / r - 1.0) / e;
        let angle = cos_angle.acos();
        let angles = [phi - angle, phi + angle];
        let p = a * (1.0 - e_squared);
        let orbit = Orbit { p, epsilon: e };
        for &angle in &angles {
            assert!(
                (orbit.r(angle) - r).abs() < 0.0001,
                "{} - {}",
                orbit.r(angle),
                r
            );
        }
        let d = [
            orbit.dx_dy(angles[0]) - (dx / dy),
            orbit.dx_dy(angles[1]) - (dx / dy),
        ];
        let angle = if d[0].abs() < d[1].abs() {
            angle
        } else {
            -angle
        };
        (orbit, angle + PI)
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

    /// Distance from center of ellipse to perihelion/aphelion.
    /// If negative, it's a hyperbola, and the distance is to perihelion.
    pub fn semi_major(&self) -> f32 {
        self.p / (1.0 - self.epsilon * self.epsilon)
    }

    pub fn dr_dphi(&self, phi: f32) -> f32 {
        let (sin, cos) = phi.sin_cos();
        let bottom = 1.0 + self.epsilon * cos;
        self.p * self.epsilon * sin / (bottom * bottom)
    }

    pub fn dx_dy(&self, phi: f32) -> f32 {
        let tan = phi.tan();
        let dr_dphi = self.dr_dphi(phi);
        let r = self.r(phi);
        // FIXME is it tan(phi * dr_dphi) or tan(phi) * dr_dphi?
        (tan * dr_dphi + r) / (dr_dphi - r * tan)
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
        let mean_motion = self.mean_motion(central_mass);
        let time_in_current_orbit = time % (TAU / mean_motion);
        let mean_anomaly = mean_motion * time_in_current_orbit;
        let mut e = mean_anomaly;
        let mut i = 0;
        loop {
            let delta =
                (e - self.epsilon * e.sin() - mean_anomaly) / (1.0 - self.epsilon * e.cos());
            if i >= 30 {
                panic!("could not converge: {}, {}", e, delta)
            }
            // HACK: half the rate of convergence means we actually converge for large epsilon
            e -= delta * 0.5;
            if delta.abs() < 1e-6 {
                return e;
            }
            i += 1;
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
