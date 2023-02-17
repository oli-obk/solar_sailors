use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::Display,
    ops::*,
    rc::{Rc, Weak},
};

use crate::save::Save;

pub struct Sensor<T> {
    input: Rc<RefCell<T>>,
}

impl<T: SetGet> Sensor<T> {
    /// Just create a sensor, leaving reader creation to later.
    pub fn raw(val: T) -> Self {
        let input = Rc::new(RefCell::new(val));
        Self { input }
    }
    /// Create a sensor and a reader at the same time
    pub fn new(val: T) -> (Self, Reader<T>) {
        let this = Self::raw(val);
        let reader = this.make_reader();
        (this, reader)
    }
    pub fn make_reader(&self) -> Reader<T> {
        let output = Rc::downgrade(&self.input);
        Reader { output }
    }
    pub fn update(&mut self, f: impl FnOnce(&mut T::Val)) {
        let mut val = self.get();
        f(&mut val);
        self.set(val);
    }
    pub fn modify(&mut self, f: impl FnOnce(T::Val) -> T::Val) -> T::Val
    where
        T::Val: Copy,
    {
        let old = self.get();
        let new = f(old);
        self.set(new);
        new
    }
}

impl<T: SetGet> SetGet for Sensor<T> {
    type Val = T::Val;
    fn set(&mut self, val: T::Val) {
        self.input.borrow_mut().set(val);
    }
    fn get(&self) -> T::Val {
        self.input.borrow().get()
    }
}

impl<T: Copy> SetGet for T {
    type Val = T;

    fn get(&self) -> Self::Val {
        *self
    }

    fn set(&mut self, val: Self::Val) {
        *self = val;
    }
}

#[derive(Clone)]
pub struct Reader<T> {
    output: Weak<RefCell<T>>,
}

pub trait SetGet {
    type Val;
    fn get(&self) -> Self::Val;
    fn set(&mut self, val: Self::Val);
}

impl<T: SetGet> Reader<T> {
    pub fn get(&self) -> Option<T::Val> {
        self.output.upgrade().map(|o| o.borrow().get())
    }
}

impl<U: AddAssign, T: SetGet<Val = U>> AddAssign<U> for Sensor<T> {
    fn add_assign(&mut self, rhs: U) {
        self.update(|val| *val += rhs)
    }
}

impl<U: MulAssign, T: SetGet<Val = U>> MulAssign<U> for Sensor<T> {
    fn mul_assign(&mut self, rhs: U) {
        self.update(|val| *val *= rhs)
    }
}

impl<U: SubAssign, T: SetGet<Val = U>> SubAssign<U> for Sensor<T> {
    fn sub_assign(&mut self, rhs: U) {
        self.update(|val| *val -= rhs)
    }
}

impl<U: RemAssign, T: SetGet<Val = U>> RemAssign<U> for Sensor<T> {
    fn rem_assign(&mut self, rhs: U) {
        self.update(|val| *val %= rhs)
    }
}

impl<U: PartialEq, T: SetGet<Val = U>> PartialEq<U> for Sensor<T> {
    fn eq(&self, other: &T::Val) -> bool {
        self.get().eq(other)
    }
}

impl<U: PartialOrd, T: SetGet<Val = U>> PartialOrd<U> for Sensor<T> {
    fn partial_cmp(&self, other: &T::Val) -> Option<Ordering> {
        self.get().partial_cmp(other)
    }
}

impl<U: Save, T: SetGet<Val = U>> Save for Sensor<T> {
    fn save(&self, key: impl Display) {
        self.get().save(key)
    }

    fn load(&mut self, key: impl Display) {
        self.update(|val| val.load(key));
    }
}
