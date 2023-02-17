use std::ops::RangeInclusive;

use macroquad::prelude::Vec2;

use crate::ship::{
    draw_gauge,
    segment::{Content, Element},
    SIZE,
};

pub struct Gauge {
    data_sources: Vec<Box<dyn FnMut() -> Option<f32>>>,
    data: Vec<f32>,
    value_range: RangeInclusive<f32>,
    handle_range: RangeInclusive<f32>,
}

impl Gauge {
    pub fn new(
        data_sources: impl IntoIterator<Item = Box<dyn FnMut() -> Option<f32>>>,
        value_range: RangeInclusive<f32>,
        handle_range: RangeInclusive<f32>,
    ) -> Self {
        let mut data_sources: Vec<_> = data_sources.into_iter().collect();
        let data = data_sources
            .iter_mut()
            .map(|f| f().unwrap_or(*value_range.start()))
            .collect();
        Self {
            data_sources,
            data,
            value_range,
            handle_range,
        }
    }
}

impl Element for Gauge {
    fn update(&mut self, _pos: Vec2) {
        for (source, dest) in self.data_sources.iter_mut().zip(&mut self.data) {
            if let Some(source) = source() {
                *dest = source;
            }
        }
    }

    fn draw(&self, pos: Vec2) {
        draw_gauge(
            pos,
            SIZE * 0.45,
            self.data.iter().copied(),
            *self.value_range.start(),
            *self.handle_range.start(),
            *self.value_range.end(),
            *self.handle_range.end(),
        );
    }
}
impl Content for Gauge {}
