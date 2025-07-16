use std::convert::TryFrom as _;
use tracing::*;
use typed_floats::{
    tf64::{
        consts::{FRAC_PI_2, PI, TAU},
        ZERO,
    },
    Atan2 as _, NonNaN, NonNaNFinite, NonZeroNonNaNFinite, Positive, PositiveFinite,
    StrictlyPositiveFinite,
};

pub use typed_floats;
pub mod orbits;

#[derive(Debug)]
pub struct Orbit {
    /// Semi-latus rectum. Basically a factor scaling the height of the ellipse.
    pub p: StrictlyPositiveFinite,
    /// Eccentricity of the orbit. Basically means how wide it is.
    /// In [0.0, 1.0) means it's an ellipse.
    /// At exactly 1.0, it's parabolic.
    /// At above 1.0 it's hyperbolic.
    // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Book%3A_Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.07%3A_Position_in_a_Hyperbolic_Orbit
    pub epsilon: PositiveFinite,
}

#[derive(Clone, Copy)]
pub enum OrbitKind {
    Circle,
    Ellipse,
    Parabola,
    Hyperbola,
}

impl OrbitKind {
    pub fn from_eccentricity(e: PositiveFinite) -> Self {
        assert!(e >= 0.0);
        if e < 1e-6 {
            Self::Circle
        } else if (e - ONE).abs() < 1e-6 {
            Self::Parabola
        } else if e < 1.0 {
            Self::Ellipse
        } else {
            Self::Hyperbola
        }
    }
}

fn square(f: NonNaN) -> PositiveFinite {
    PositiveFinite::try_from(f * f).unwrap()
}

const ONE: StrictlyPositiveFinite = match StrictlyPositiveFinite::<f64>::new(1.0) {
    Ok(val) => val,
    Err(_) => panic!(),
};
const TWO: StrictlyPositiveFinite = match StrictlyPositiveFinite::<f64>::new(2.0) {
    Ok(val) => val,
    Err(_) => panic!(),
};
const THREE: StrictlyPositiveFinite = match StrictlyPositiveFinite::<f64>::new(3.0) {
    Ok(val) => val,
    Err(_) => panic!(),
};

impl Orbit {
    pub fn circular(radius: StrictlyPositiveFinite) -> Self {
        Self {
            p: radius,
            epsilon: ZERO.into(),
        }
    }

    /// Compute orbit from position and speed. The second return value is the angle of the orbit.
    /// The third return value is the starting time of the object in the orbit.
    #[instrument(level = "debug")]
    pub fn from_pos_dir(
        x: NonNaN,
        y: NonNaN,
        dx: NonNaN,
        dy: NonNaN,
    ) -> (Self, NonNaNFinite, PositiveFinite) {
        let r_squared = StrictlyPositiveFinite::try_from(square(x) + square(y)).unwrap();
        let r = r_squared.sqrt();
        let phi = y.atan2(x);
        let v_squared = square(dx) + square(dy);
        let a_over_r = ONE / NonZeroNonNaNFinite::try_from(TWO - r * v_squared).unwrap();
        let a = NonZeroNonNaNFinite::try_from(r * a_over_r).unwrap();
        let xi = dy.atan2(dx);
        let sinxiphi = NonNaNFinite::try_from(xi - phi).unwrap().sin();
        let sinxiphi2 = square(sinxiphi.into());
        // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Book%3A_Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector
        // formula 9.9.4
        let rvs = Positive::try_from(r_squared * v_squared * sinxiphi2).unwrap();
        let one_neg_e_squared = rvs / a;
        let e_squared = StrictlyPositiveFinite::try_from(ONE - one_neg_e_squared).unwrap();
        let e = e_squared.sqrt();
        let cos_angle = StrictlyPositiveFinite::try_from((rvs / r - ONE) / e).unwrap();
        trace!(?cos_angle);
        // Truncate precision to f32 to make sure we never get above 1.0 even with some float math issues
        let angle = StrictlyPositiveFinite::<f64>::new(f64::from(cos_angle) as f32 as f64)
            .unwrap()
            .acos();
        assert!(angle.abs() <= 1.0);
        let angle = PositiveFinite::try_from(angle).unwrap();
        let angles = [
            NonNaNFinite::try_from(phi - angle).unwrap(),
            NonNaNFinite::try_from(phi + angle).unwrap(),
        ];
        trace!(?angles);
        let kind = OrbitKind::from_eccentricity(e.into());
        let p = match kind {
            OrbitKind::Circle => StrictlyPositiveFinite::try_from(a).unwrap(),
            OrbitKind::Ellipse => StrictlyPositiveFinite::try_from(a * (ONE - e_squared)).unwrap(),
            OrbitKind::Parabola => StrictlyPositiveFinite::try_from((cos_angle + ONE) * r).unwrap(),
            OrbitKind::Hyperbola => {
                StrictlyPositiveFinite::try_from(-a * (e_squared - ONE)).unwrap()
            }
        };
        let orbit = Orbit {
            p,
            epsilon: e.into(),
        };
        for &angle in &angles {
            assert!(
                (orbit.r(angle) - r).abs() < 1e-5,
                "{} - {}",
                orbit.r(angle),
                r
            );
        }
        let (t, angle) = if let OrbitKind::Circle = kind {
            // Circle
            (ZERO.into(), phi)
        } else {
            // xi is angle of direction
            // HACK: we treat the tangent as 90° to the orbital angular position.
            let d = [
                NonNaNFinite::try_from(FRAC_PI_2 - xi).unwrap(),
                NonNaNFinite::try_from(-FRAC_PI_2 - xi).unwrap(),
            ];
            let d = |i, j| {
                NonNaNFinite::<f64>::abs(
                    NonNaNFinite::try_from(angles[i] + d[j] + TAU).unwrap() % TAU,
                )
            };
            let d = [d(0, 0).min(d(0, 1)), d(1, 0).min(d(1, 1))];
            let angle = if d[0] < d[1] {
                angle - d[0]
            } else {
                NonNaNFinite::try_from(angle + d[1]).unwrap()
            };
            // 9.8.1
            let cos_big_e = (e + cos_angle) / (rvs / r);
            // Truncate precision to f32 to make sure we never get above 1.0 even with some float math issues
            let big_e = NonNaNFinite::try_from(
                StrictlyPositiveFinite::try_from((f64::from(cos_big_e) as f32) as f64)
                    .unwrap()
                    .acos(),
            )
            .unwrap();
            let big_e = if angle < PI {
                big_e
            } else {
                NonNaNFinite::try_from(TAU - big_e).unwrap()
            };
            let m = big_e - e * big_e.sin();
            let mean_motion = orbit.mean_motion();
            (PositiveFinite::try_from(m / mean_motion).unwrap(), angle)
        };

        (orbit, angle, t)
    }

    /// Radius at orbital angle `phi` in orbit coordinates, not in the coordinate system of the center of gravity.
    /// You need to adjust for the angle of the orbit yourself.
    pub fn r(&self, phi: NonNaNFinite) -> NonNaN {
        self.p / NonZeroNonNaNFinite::try_from(ONE + self.epsilon * phi.cos()).unwrap()
    }

    /// Radius at the point closest to the center of gravity.
    pub fn perihelion(&self) -> NonNaN {
        self.r(ZERO.into())
    }

    /// Radius at the point farthest away from the center of gravity.
    pub fn aphelion(&self) -> NonNaN {
        self.r(PI.into())
    }

    /// Distance from center of ellipse to perihelion/aphelion.
    /// If it's a hyperbola or parabola, the distance is from center of gravity to perihelion.
    pub fn semi_major(&self) -> StrictlyPositiveFinite {
        StrictlyPositiveFinite::try_from(self.p / self.eps_squared()).unwrap()
    }

    fn eps_squared(&self) -> StrictlyPositiveFinite {
        match self.kind() {
            OrbitKind::Circle => ONE,
            OrbitKind::Ellipse => {
                StrictlyPositiveFinite::try_from(ONE - square(self.epsilon.into())).unwrap()
            }
            OrbitKind::Parabola => TWO,
            OrbitKind::Hyperbola => {
                StrictlyPositiveFinite::try_from(square(self.epsilon.into()) - ONE).unwrap()
            }
        }
    }

    /// Distance from the center of the ellipse to the point at 90° to the semi major axis
    pub fn semi_minor(&self) -> StrictlyPositiveFinite {
        match self.kind() {
            OrbitKind::Circle => self.p,
            OrbitKind::Ellipse => StrictlyPositiveFinite::try_from(
                self.p
                    / StrictlyPositiveFinite::try_from((ONE - square(self.epsilon.into())).sqrt())
                        .unwrap(),
            )
            .unwrap(),
            OrbitKind::Parabola => panic!("cannot compute semi minor axis for for parabola"),
            OrbitKind::Hyperbola => panic!("cannot compute semi minor axis for for hyperbola"),
        }
    }

    pub fn area(&self) -> StrictlyPositiveFinite {
        StrictlyPositiveFinite::try_from(PI * self.semi_major() * self.semi_minor()).unwrap()
    }

    pub fn mean_motion(&self) -> StrictlyPositiveFinite {
        let semi_major = self.semi_major();
        StrictlyPositiveFinite::try_from((ONE / semi_major).sqrt() / semi_major).unwrap()
    }

    /// This cannot be solved numerically, we loop until the precision is
    /// in the 1e-6 range. Formula from https://space.stackexchange.com/questions/8911/determining-orbital-position-at-a-future-point-in-time
    #[instrument(level = "trace")]
    pub fn eccentric_anomaly(&self, time: PositiveFinite) -> NonNaNFinite {
        let mean_motion = self.mean_motion();
        let time_in_current_orbit = if self.epsilon < 1.0 {
            // Optimize repeating orbits by only computing the
            // position from the last apehelion crossing.
            time % StrictlyPositiveFinite::try_from(TAU / mean_motion).unwrap()
        } else {
            time
        };
        let mean_anomaly = PositiveFinite::try_from(mean_motion * time_in_current_orbit).unwrap();
        let mut e = NonNaNFinite::from(mean_anomaly);
        let mut i = 0;
        loop {
            let old = e;
            match self.kind() {
                OrbitKind::Circle => unreachable!(),
                OrbitKind::Ellipse => {
                    // 9.6.8
                    // E = (M - e(E*cos(E) - sin(E)))/(1 - e * cos(E))
                    let sin = e.sin();
                    let cos = e.cos();
                    e = NonNaNFinite::try_from(
                        (mean_anomaly
                            - NonNaNFinite::try_from(self.epsilon * (e * cos - sin)).unwrap())
                            / NonZeroNonNaNFinite::try_from(ONE - self.epsilon * cos).unwrap(),
                    )
                    .unwrap();
                }
                OrbitKind::Parabola => {
                    let u2 = square(e.into());
                    let c = mean_anomaly
                        * PI
                        * StrictlyPositiveFinite::try_from((18.0_f64).sqrt()).unwrap();
                    e = NonNaNFinite::try_from((TWO * u2 + c) / (THREE + THREE * u2)).unwrap();
                }
                OrbitKind::Hyperbola => {
                    // 9.8.14
                    let cosh = e.cosh();
                    let sinh = e.sinh();
                    // E = (M + e(E*cosh(E) - sinh(E)))/(e * cosh(E) - 1)
                    e = NonNaNFinite::try_from(
                        mean_anomaly
                            + PositiveFinite::try_from(
                                PositiveFinite::try_from(
                                    self.epsilon
                                        * (PositiveFinite::try_from(e * cosh).unwrap() - sinh),
                                )
                                .unwrap()
                                    / (PositiveFinite::try_from(self.epsilon * cosh).unwrap()
                                        - ONE),
                            )
                            .unwrap(),
                    )
                    .unwrap();
                }
            }
            let delta = e - old;
            trace!(?delta);
            if i >= 30 {
                trace!(
                    ?e,
                    ?delta,
                    "could not converge, aborting after 30 iterations with imprecise result"
                );
                return e;
            }
            if delta.abs() < 1e-6 {
                return e;
            }
            i += 1;
        }
    }

    /// The angle of the object after `time` seconds, when starting at angle `0`
    pub fn angle_at(&self, time: PositiveFinite) -> NonNaNFinite {
        match self.kind() {
            OrbitKind::Circle => {
                let mean_motion = self.mean_motion();
                // FIXME: eliminate the cancelling out of TAU in the
                // math below.
                let period = TAU / mean_motion;
                let time = PositiveFinite::try_from(time % period).unwrap();
                NonNaNFinite::try_from(TAU * time / period).unwrap()
            }
            OrbitKind::Ellipse => {
                let e = self.eccentric_anomaly(time);
                let x = e.cos() - self.epsilon;
                let y = e.sin() * self.eps_squared().sqrt();
                y.atan2(x)
            }
            OrbitKind::Parabola => {
                let e = self.eccentric_anomaly(time);
                let u2 = e * e;
                let cosv = NonNaNFinite::try_from((ONE - u2) / (ONE + u2)).unwrap();
                NonNaNFinite::try_from(cosv.acos()).unwrap()
            }
            OrbitKind::Hyperbola => {
                let e = self.eccentric_anomaly(time);
                // 9.8.6
                // cos(v) = (e - cosh(E))/(e * cosh(E) - 1)
                let cosh = e.cosh();
                let cosv = NonNaNFinite::try_from(
                    (self.epsilon - cosh)
                        / (PositiveFinite::try_from(self.epsilon * cosh).unwrap() - ONE),
                )
                .unwrap();
                NonNaNFinite::try_from(NonNaNFinite::try_from(cosv.acos()).unwrap() * time.signum())
                    .unwrap()
            }
        }
    }

    pub fn kind(&self) -> OrbitKind {
        OrbitKind::from_eccentricity(self.epsilon)
    }
}
