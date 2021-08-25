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
                speed: 0.01,
            },
            reader,
        )
    }

    pub fn control(&mut self, dir: bool) {
        if !dir {
            self.value.update(|v| self.min.max(v - self.speed));
        } else {
            self.value.update(|v| self.max.min(v + self.speed));
        }
    }
}
