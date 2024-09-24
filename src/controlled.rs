use std::ops::{Add, Sub};

use macroquad::prelude::*;

use crate::datastructures::{Reader, Sensor, SetGet};

pub(crate) struct ControlledRange<T = f32> {
    pub min: T,
    pub value: Sensor<T>,
    pub max: T,
    speed: T,
}

impl<T: SetGet<Val = T> + Add<Output = T> + Sub<Output = T> + Copy + From<f32> + PartialOrd>
    ControlledRange<T>
{
    pub fn new(min: T, max: T) -> (Self, Reader<T>) {
        let (value, reader) = Sensor::new(max);
        (
            Self {
                value,
                min,
                max,
                speed: T::from(0.1),
            },
            reader,
        )
    }

    pub fn control(&mut self, dir: bool) {
        let Self {
            ref mut value,
            min,
            max,
            speed,
            ..
        } = *self;
        if !dir {
            value.modify(|v| if v - speed < min { min } else { v - speed });
        } else {
            value.modify(|v| if v + speed > max { max } else { v + speed });
        }
    }
}
