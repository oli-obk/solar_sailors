use std::{cell::RefCell, rc::Rc};

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

impl<T: Element + ?Sized> Element for Rc<RefCell<T>> {
    fn update(&mut self) {
        self.borrow_mut().update()
    }

    fn draw(&self, pos: Vec2) {
        self.borrow().draw(pos)
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
pub trait Attachement: Element {}

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
        for attachement in &self.attachements {
            attachement.draw(pos);
        }
    }
}

pub const SIZE: f32 = 40.0;

fn hexagon_offset(position: Position) -> Vec2 {
    let x = SIZE;
    let x2 = x / 2.0;
    match position {
        Position::Up => vec2(0.0, -x),
        Position::RightUp => vec2(x2, -x2),
        Position::RightDown => vec2(x2, x2),
        Position::Down => vec2(0.0, x),
        Position::LeftDown => vec2(-x2, x2),
        Position::LeftUp => vec2(-x2, -x2),
    }
}

/// Sides of a Hexagon, in clockwise order
pub enum Position {
    Up = 0,
    RightUp = 1,
    RightDown = 2,
    Down = 3,
    LeftDown = 4,
    LeftUp = 5,
}
