use macroquad::prelude::*;

pub(crate) struct ButtonControlledRange {
    pub value: f32,
    keycode: KeyCode,
}

impl ButtonControlledRange {
    pub fn new(start: f32, keycode: KeyCode) -> Self {
        Self {
            value: start,
            keycode,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(self.keycode) {
            if is_key_down(KeyCode::LeftShift) {
                self.value -= 1.0;
            } else {
                self.value += 1.0;
            }
        }
    }
}