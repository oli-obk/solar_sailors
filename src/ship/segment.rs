use std::{
    cell::RefCell,
    f32::consts::{FRAC_PI_3, PI},
    rc::Rc,
};

use macroquad::prelude::*;

pub trait Element {
    fn update(&mut self);
    fn draw(&self, pos: Vec2);
}

impl<T: Element> Element for Option<T> {
    fn update(&mut self) {
        if let Some(this) = self {
            this.update()
        }
    }

    fn draw(&self, pos: Vec2) {
        if let Some(this) = self {
            this.draw(pos)
        }
    }
}

impl<T: Attachement> Attachement for Option<T> {
    fn update(&mut self) {
        if let Some(this) = self {
            this.update()
        }
    }

    fn draw(&self, pos: Vec2, angle: f32) {
        if let Some(this) = self {
            this.draw(pos, angle)
        }
    }
}

impl<T: Attachement + ?Sized> Attachement for Rc<RefCell<T>> {
    fn update(&mut self) {
        self.borrow_mut().update()
    }

    fn draw(&self, pos: Vec2, angle: f32) {
        self.borrow().draw(pos, angle)
    }
}

impl<T: Element + ?Sized> Element for Box<T> {
    fn update(&mut self) {
        (&mut **self).update()
    }

    fn draw(&self, pos: Vec2) {
        (&**self).draw(pos)
    }
}

pub trait Content: Element {}
pub trait Attachement {
    fn update(&mut self);
    fn draw(&self, pos: Vec2, angle: f32);
}

#[derive(Default)]
pub struct Segment {
    pub content: Option<Box<dyn Content>>,
    pub attachements: [Option<Rc<RefCell<dyn Attachement>>>; 6],
}

impl Element for Segment {
    fn update(&mut self) {
        self.content.update();
        for attachement in &mut self.attachements {
            attachement.update();
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

const ATTACHEMENT_ANGLES: [f32; 6] = [
    0.0,
    FRAC_PI_3,
    FRAC_PI_3 * 2.0,
    PI,
    FRAC_PI_3 * 4.0,
    FRAC_PI_3 * 5.0,
];

pub const SIZE: f32 = 40.0;

const ATTACHEMENT_OFFSETS: [Vec2; 6] = {
    let x = SIZE;
    let x2 = x / 2.0;
    [
        const_vec2!([0.0, -x]),
        const_vec2!([x2, -x2]),
        const_vec2!([x2, x2]),
        const_vec2!([0.0, x]),
        const_vec2!([-x2, x2]),
        const_vec2!([-x2, -x2]),
    ]
};
