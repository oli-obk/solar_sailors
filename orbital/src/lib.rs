use std::f64::consts::{PI, TAU};

#[derive(Debug)]
pub struct Orbit {
    /// Semi-latus rectum. Basically a factor scaling the height of the ellipse.
    pub p: f64,
    /// Eccentricity of the orbit. Basically means how wide it is.
    /// In [0.0, 1.0) means it's an ellipse.
    /// At exactly 1.0, it's parabolic.
    /// At above 1.0 it's hyperbolic.
    // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Book%3A_Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.07%3A_Position_in_a_Hyperbolic_Orbit
    pub epsilon: f64,
}

impl Orbit {
    pub fn circular(radius: f64) -> Self {
        Self {
            p: radius,
            epsilon: 0.0,
        }
    }

    /// Compute orbit from position and speed. The second return value is the angle of the orbit.
    /// The third return value is the starting time of the object in the orbit.
    pub fn from_pos_dir(x: f64, y: f64, dx: f64, dy: f64) -> (Self, f64, f64) {
        let x = x as f64;
        let y = y as f64;
        let dx = dx as f64;
        let dy = dy as f64;
        let r_squared = x * x + y * y;
        let r = r_squared.sqrt();
        let phi = y.atan2(x);
        let v_squared = dx * dx + dy * dy;
        let a_over_r = 1.0 / (2.0 - r * v_squared);
        let a = r * a_over_r;
        let xi = dy.atan2(dx);
        let sinxiphi = (xi - phi).sin();
        let sinxiphi2 = sinxiphi * sinxiphi;
        // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Book%3A_Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector
        // formula 9.9.4
        let rvs = r_squared * v_squared * sinxiphi2;
        let one_neg_e_squared = rvs / a;
        let e_squared = 1.0 - one_neg_e_squared;
        let e = e_squared.sqrt();
        let cos_angle = (rvs / r - 1.0) / e;
        // Truncate precision to f32 to make sure we never get above 1.0 even with some float math issues
        let angle = ((cos_angle as f32) as f64).acos();
        let angles = [phi - angle, phi + angle];
        let (p, r) = if e < 1.0 {
            (a * (1.0 - e_squared), r)
        } else {
            // hyperbolic orbit
            (a * (e_squared - 1.0), -r)
        };
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
            TAU - angle
        };
        // 9.8.1
        let cos_big_e = (e + cos_angle) / (rvs / r);
        // Truncate precision to f32 to make sure we never get above 1.0 even with some float math issues
        let big_e = ((cos_big_e as f32) as f64).acos();
        let big_e = if angle < PI { big_e } else { TAU - big_e };
        let m = big_e - e * big_e.sin();
        let mean_motion = orbit.mean_motion(1.0);
        let t = m / mean_motion;

        let angle = if e < 1.0 { angle } else { angle + PI };
        (orbit, angle, t)
    }

    /// Radius at orbital angle `phi` in orbit coordinates, not in the coordinate system of the center of gravity.
    /// You need to adjust for the angle of the orbit yourself.
    pub fn r(&self, phi: f64) -> f64 {
        self.p / (1.0 + self.epsilon * phi.cos())
    }

    /// Radius at the point closest to the center of gravity.
    pub fn perihelion(&self) -> f64 {
        self.r(0.0)
    }

    /// Radius at the point farthest away from the center of gravity.
    pub fn aphelion(&self) -> f64 {
        self.r(PI)
    }

    /// Distance from center of ellipse to perihelion/aphelion.
    /// If it's a hyperbola, the distance is to perihelion.
    pub fn semi_major(&self) -> f64 {
        if self.epsilon < 1.0 {
        self.p / (1.0 - self.epsilon * self.epsilon)
        } else {
            -self.p / (self.epsilon * self.epsilon - 1.0)
        }
    }

    pub fn dr_dphi(&self, phi: f64) -> f64 {
        let (sin, cos) = phi.sin_cos();
        let bottom = 1.0 + self.epsilon * cos;
        self.p * self.epsilon * sin / (bottom * bottom)
    }

    pub fn dx_dy(&self, phi: f64) -> f64 {
        let tan = phi.tan();
        let dr_dphi = self.dr_dphi(phi);
        let r = self.r(phi);
        // FIXME is it tan(phi * dr_dphi) or tan(phi) * dr_dphi?
        (tan * dr_dphi + r) / (dr_dphi - r * tan)
    }

    /// Distance from the center of the ellipse to the point at 90° to the semi major axis
    pub fn semi_minor(&self) -> f64 {
        self.p / (1.0 - self.epsilon * self.epsilon).sqrt()
    }

    pub fn area(&self) -> f64 {
        PI * self.semi_major() * self.semi_minor()
    }

    pub fn mean_motion(&self, central_mass: f64) -> f64 {
        let semi_major = self.semi_major();
        let specific_orbital_energy = -central_mass / (2.0 * semi_major);
        (-2.0 * specific_orbital_energy).sqrt() / semi_major
    }

    /// This cannot be solved numerically, we loop until the precision is
    /// in the 1e-6 range. Formula from https://space.stackexchange.com/questions/8911/determining-orbital-position-at-a-future-point-in-time
    pub fn eccentric_anomaly(&self, central_mass: f64, time: f64) -> f64 {
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
    pub fn angle_at(&self, central_mass: f64, time: f64) -> f64 {
        let e = self.eccentric_anomaly(central_mass, time);
        let x = e.cos() - self.epsilon;
        let y = e.sin() * (1.0 - self.epsilon * self.epsilon).sqrt();
        y.atan2(x)
    }
}
