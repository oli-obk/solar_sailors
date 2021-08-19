use macroquad::prelude::*;

use crate::datastructures::{Reader, Sensor};

pub(crate) struct ButtonControlledRange {
    pub min: f32,
    pub value: Sensor<f32>,
    pub max: f32,
    speed: f32,
    keycode: KeyCode,
}

impl ButtonControlledRange {
    pub fn new(min: f32, max: f32, keycode: KeyCode) -> (Self, Reader<f32>) {
        let (value, reader) = Sensor::new(max);
        (
            Self {
                value,
                keycode,
                min,
                max,
                speed: 0.01,
            },
            reader,
        )
    }

    pub fn update(&mut self) {
        if is_key_down(self.keycode) {
            if is_key_down(KeyCode::LeftShift) {
                self.value.update(|v| self.min.max(v - self.speed));
            } else {
                self.value.update(|v| self.max.min(v + self.speed));
            }
        }
    }
}
