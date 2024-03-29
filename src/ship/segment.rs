use std::f32::consts::{FRAC_PI_3, PI};

use macroquad::prelude::*;

pub trait Element {
    fn update(&mut self, pos: Vec2);
    fn draw(&self, pos: Vec2);
}

impl<T: Element> Element for Option<T> {
    fn update(&mut self, pos: Vec2) {
        if let Some(this) = self {
            this.update(pos)
        }
    }

    fn draw(&self, pos: Vec2) {
        if let Some(this) = self {
            this.draw(pos)
        }
    }
}

impl<T: Attachement> Attachement for Option<T> {
    fn update(&mut self, pos: Vec2, angle: f32) {
        if let Some(this) = self {
            this.update(pos, angle)
        }
    }

    fn control(&mut self, dir: Option<bool>, pos: Option<f32>) {
        if let Some(this) = self {
            this.control(dir, pos)
        }
    }

    fn draw(&self, pos: Vec2, angle: f32) {
        if let Some(this) = self {
            this.draw(pos, angle)
        }
    }
}

impl<T: Attachement + ?Sized> Attachement for Box<T> {
    fn update(&mut self, pos: Vec2, angle: f32) {
        T::update(self, pos, angle)
    }

    fn control(&mut self, dir: Option<bool>, pos: Option<f32>) {
        T::control(self, dir, pos)
    }

    fn draw(&self, pos: Vec2, angle: f32) {
        T::draw(self, pos, angle)
    }
}

impl<T: Element + ?Sized> Element for Box<T> {
    fn update(&mut self, pos: Vec2) {
        (**self).update(pos)
    }

    fn draw(&self, pos: Vec2) {
        (**self).draw(pos)
    }
}

pub trait Content: Element {}
pub trait Attachement {
    fn update(&mut self, pos: Vec2, angle: f32);
    fn control(&mut self, dir: Option<bool>, pos: Option<f32>);
    fn draw(&self, pos: Vec2, angle: f32);
}

#[derive(Default)]
pub struct Segment {
    pub content: Option<Box<dyn Content>>,
    pub attachements: [Option<Box<dyn Attachement>>; 6],
}

impl Element for Segment {
    fn update(&mut self, pos: Vec2) {
        self.content.update(pos);
        for (i, attachement) in self.attachements.iter_mut().enumerate() {
            attachement.update(pos + ATTACHEMENT_OFFSETS[i], ATTACHEMENT_ANGLES[i]);
        }
    }

    fn draw(&self, pos: Vec2) {
        draw_hexagon(
            pos.x,
            pos.y,
            SIZE / (3.0_f32).sqrt(),
            1.0,
            false,
            DARKBLUE,
            BLUE,
        );
        self.content.draw(pos);
        for (i, attachement) in self.attachements.iter().enumerate() {
            attachement.draw(pos + ATTACHEMENT_OFFSETS[i], ATTACHEMENT_ANGLES[i]);
        }
    }
}

pub(crate) const ATTACHEMENT_ANGLES: [f32; 6] = [
    0.0,
    FRAC_PI_3,
    FRAC_PI_3 * 2.0,
    PI,
    FRAC_PI_3 * 4.0,
    FRAC_PI_3 * 5.0,
];

pub const SIZE: f32 = 100.0;

pub(crate) const ATTACHEMENT_OFFSETS: [Vec2; 6] = {
    let x = SIZE / 2.0;
    let x2 = x / 2.0;
    let x3 = x * 0.866_025_4; // (x * 0.75).sqrt()
    [
        Vec2::from_array([0.0, -x]),
        Vec2::from_array([x3, -x2]),
        Vec2::from_array([x3, x2]),
        Vec2::from_array([0.0, x]),
        Vec2::from_array([-x3, x2]),
        Vec2::from_array([-x3, -x2]),
    ]
};
