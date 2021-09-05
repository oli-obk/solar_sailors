use macroquad::prelude::*;

use crate::datastructures::{Reader, Sensor};

pub(crate) struct ControlledRange {
    pub min: f32,
    pub value: Sensor<f32>,
    pub max: f32,
    speed: f32,
}

impl ControlledRange {
    pub fn new(min: f32, max: f32) -> (Self, Reader<f32>) {
        let (value, reader) = Sensor::new(max);
        (
            Self {
                value,
                min,
                max,
                speed: 0.1,
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
            value.modify(|v| min.max(v - speed));
        } else {
            value.modify(|v| max.min(v + speed));
        }
    }
}
