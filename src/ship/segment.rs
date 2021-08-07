use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

trait Element {
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

trait Content: Element {}
trait Attachement: Element {}

#[derive(Default)]
pub struct Segment {
    content: Option<Box<dyn Content>>,
    attachements: [Option<Rc<RefCell<dyn Attachement>>>; 6],
}

impl Segment {
    pub fn update(&mut self) {
        self.content.update();
        for attachement in &mut self.attachements {
            attachement.update();
        }
    }

    pub fn draw(&self, pos: Vec2) {
        draw_poly(pos.x, pos.y, 6, 40.0 / (3.0_f32).sqrt(), 0.0, BLUE);
        draw_poly_lines(pos.x, pos.y, 6, 40.0 / (3.0_f32).sqrt(), 0.0, 1.0, DARKBLUE);
        self.content.draw(pos);
        for attachement in &self.attachements {
            attachement.draw(pos);
        }
    }
}
