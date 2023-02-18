use std::ops::RangeInclusive;

use macroquad::prelude::Vec2;

use crate::ship::{
    draw_gauge,
    segment::{Content, Element},
    SIZE,
};

pub enum GaugeHandle {
    Absolute(f32),
    // Relative to the previous gauge handle in the list
    Relative(f32),
}

impl From<f32> for GaugeHandle {
    fn from(v: f32) -> Self {
        Self::Absolute(v)
    }
}
impl GaugeHandle {
    fn make_absolute(self, prev: f32) -> f32 {
        match self {
            GaugeHandle::Absolute(v) => v,
            GaugeHandle::Relative(r) => prev + r,
        }
    }
}

pub struct Gauge {
    data_sources: Vec<Box<dyn FnMut(f32) -> Option<GaugeHandle>>>,
    data: Vec<f32>,
    value_range: RangeInclusive<f32>,
    handle_range: RangeInclusive<f32>,
}

impl Gauge {
    pub fn new(
        data_sources: impl IntoIterator<Item = Box<dyn FnMut(f32) -> Option<GaugeHandle>>>,
        value_range: RangeInclusive<f32>,
        handle_range: RangeInclusive<f32>,
    ) -> Self {
        let mut data_sources: Vec<_> = data_sources.into_iter().collect();
        let data = data_sources
            .iter_mut()
            .map(|f| f(0.0).map_or(*value_range.start(), |v| v.make_absolute(0.0)))
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
        let mut prev = *self.value_range.start();
        let mut prev_diff = 0.0;
        for (source, dest) in self.data_sources.iter_mut().zip(&mut self.data) {
            if let Some(source) = source(prev_diff) {
                let new = source.make_absolute(prev);
                prev_diff = new - *dest;
                (*dest, prev) = (new, *dest);
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
