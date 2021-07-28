use macroquad::prelude::*;

pub(crate) struct ButtonControlledRange {
    pub min: f32,
    pub max: f32,
    keycode: KeyCode,
}

impl ButtonControlledRange {
    pub fn new(max: f32, keycode: KeyCode) -> Self {
        Self {
            keycode,
            min: 0.0,
            max,
        }
    }

    pub fn apply(&self, value: f32, step: f32) -> f32 {
        if !is_key_down(self.keycode) {
            return value;
        }
        let new = if is_key_down(KeyCode::LeftShift) {
            value - step
        } else {
            value + step
        };
        println!("{} + {}", value, step);
        new.min(self.max).max(self.min)
    }
}
