use std::{
    cell::Cell,
    rc::{Rc, Weak},
};

pub struct Sensor<T: Copy> {
    input: Rc<Cell<T>>,
}

impl<T: Copy> Sensor<T> {
    pub fn new(val: T) -> (Self, Reader<T>) {
        let input = Rc::new(Cell::new(val));
        let output = Rc::downgrade(&input);
        (Self { input }, Reader { output })
    }
    pub fn set(&self, val: T) {
        self.input.set(val);
    }
    pub fn get(&self) -> T {
        self.input.get()
    }
}

pub struct Reader<T> {
    output: Weak<Cell<T>>,
}

impl<T: Copy> Reader<T> {
    pub fn get(&self) -> Option<T> {
        self.output.upgrade().map(|o| o.get())
    }
}
