use macroquad::prelude::*;

pub(crate) struct ButtonControlledRange {
    pub min: f32,
    pub value: f32,
    pub max: f32,
    keycode: KeyCode,
}

impl ButtonControlledRange {
    pub fn new(min: f32, max: f32, keycode: KeyCode) -> Self {
        Self {
            value: max,
            keycode,
            min,
            max,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(self.keycode) {
            if is_key_down(KeyCode::LeftShift) {
                self.value = self.min.max(self.value - 1.0);
            } else {
                self.value = self.max.min(self.value + 1.0);
            }
        }
    }
}
