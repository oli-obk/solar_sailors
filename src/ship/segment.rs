use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicI64, Ordering},
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

impl<T: Element + ?Sized> Element for Rc<SharedElement<T>> {
    fn update(&mut self) {
        (&**self).update()
    }

    fn draw(&self, pos: Vec2) {
        (&**self).draw(pos)
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
    pub attachements: [Option<Rc<SharedElement<dyn Attachement>>>; 6],
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
            40.0 / (3.0_f32).sqrt(),
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

pub struct Weld {
    pub elements: [Rc<SharedElement<Segment>>; 2],
    pub position: Position,
}

impl Attachement for Weld {}

impl Element for Weld {
    fn update(&mut self) {
        for element in &mut self.elements {
            element.update()
        }
    }

    fn draw(&self, pos: Vec2) {
        let x = 40.0;
        let x2 = x / 2.0;
        let offset = match self.position {
            Position::Up => vec2(0.0, -x),
            Position::RightUp => vec2(x2, -x2),
            Position::RightDown => vec2(x2, x2),
            Position::Down => vec2(0.0, x),
            Position::LeftDown => vec2(-x2, x2),
            Position::LeftUp => vec2(-x2, -x2),
        };
        self.elements[0].draw(pos - offset);
        self.elements[1].draw(pos + offset);
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

pub struct SharedElement<T: ?Sized> {
    inner: RefCell<SharedElementInner<T>>,
}

struct SharedElementInner<T: ?Sized> {
    frame_counter: i64,
    element: T,
}

impl<T: ?Sized> SharedElement<T> {
    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.inner.borrow_mut().element)
    }
    pub fn borrow(&self, f: impl FnOnce(&T)) {
        f(&self.inner.borrow().element)
    }
}

impl<T> SharedElement<T> {
    pub fn new(element: T) -> Rc<Self> {
        Rc::new(Self {
            inner: RefCell::new(SharedElementInner {
                frame_counter: FRAME.load(Ordering::Relaxed).into(),
                element,
            }),
        })
    }
}

static FRAME: AtomicI64 = AtomicI64::new(0);

pub fn start_next_frame() {
    FRAME.fetch_add(1, Ordering::Relaxed);
}

impl<T: Element + ?Sized> SharedElement<T> {
    pub fn update(&self) {
        let frame = FRAME.load(Ordering::Relaxed);
        let mut inner = match self.inner.try_borrow_mut() {
            Ok(inner) => inner,
            // Already within this function somewhere higher on the stack
            Err(_) => return,
        };
        if inner.frame_counter.abs() == frame {
            return;
        }
        assert_eq!(frame, inner.frame_counter + 1);
        // Using sign as marker for "still need to draw"
        inner.frame_counter = -frame;
        inner.element.update();
    }

    pub fn draw(&self, pos: Vec2) {
        let mut inner = match self.inner.try_borrow_mut() {
            Ok(inner) => inner,
            // Already within this function somewhere higher on the stack
            Err(_) => return,
        };
        if inner.frame_counter >= 0 {
            // Already drawn
            return;
        }
        inner.frame_counter = -inner.frame_counter;
        assert_eq!(inner.frame_counter, FRAME.load(Ordering::Relaxed));
        inner.element.draw(pos)
    }
}
