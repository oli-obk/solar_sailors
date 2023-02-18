use std::ops::RangeInclusive;

use macroquad::prelude::Vec2;

use crate::ship::{
    segment::{Content, Element},
    SIZE,
};

pub enum GaugeHandleKind {
    Absolute,
    // Relative to the previous gauge handle in the list
    Relative,
}

pub struct GaugeHandle {
    pub kind: GaugeHandleKind,
    source: Box<dyn FnMut(f32) -> Option<f32>>,
}

impl<T: FnMut(f32) -> Option<f32> + 'static> From<T> for GaugeHandle {
    fn from(value: T) -> Self {
        GaugeHandle {
            kind: GaugeHandleKind::Absolute,
            source: Box::new(value),
        }
    }
}

impl GaugeHandle {
    pub fn relative(self) -> Self {
        Self {
            kind: GaugeHandleKind::Relative,
            ..self
        }
    }
}

pub struct Gauge {
    pub data_sources: Vec<GaugeHandle>,
    pub data: Vec<f32>,
    pub value_range: RangeInclusive<f32>,
    pub handle_range: RangeInclusive<f32>,
}

impl Gauge {
    pub fn new(
        data_sources: impl IntoIterator<Item = GaugeHandle>,
        value_range: RangeInclusive<f32>,
        handle_range: RangeInclusive<f32>,
    ) -> Self {
        let mut data_sources: Vec<_> = data_sources.into_iter().collect();
        let data = data_sources
            .iter_mut()
            .map(|gh| (gh.source)(0.0).unwrap_or(*value_range.start()))
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
        let mut prev_diff = 0.0;
        for (gh, dest) in self.data_sources.iter_mut().zip(&mut self.data) {
            if let Some(new) = (gh.source)(prev_diff) {
                prev_diff = new - *dest;
                *dest = new;
            }
        }
    }

    fn draw(&self, pos: Vec2) {
        self.draw_inner(pos, SIZE * 0.45);
    }
}
impl Content for Gauge {}
