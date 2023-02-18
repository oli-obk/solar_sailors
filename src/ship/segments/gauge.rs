use std::ops::RangeInclusive;

use macroquad::prelude::Vec2;

use crate::ship::{
    draw_gauge,
    segment::{Content, Element},
    SIZE,
};

enum GaugeHandleKind {
    Absolute,
    // Relative to the previous gauge handle in the list
    Relative,
}

pub struct GaugeHandle {
    kind: GaugeHandleKind,
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
    data_sources: Vec<GaugeHandle>,
    data: Vec<f32>,
    value_range: RangeInclusive<f32>,
    handle_range: RangeInclusive<f32>,
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
        let mut prev = *self.value_range.start();
        let mut prev_diff = 0.0;
        for (gh, dest) in self.data_sources.iter_mut().zip(&mut self.data) {
            if let Some(source) = (gh.source)(prev_diff) {
                let new = match gh.kind {
                    GaugeHandleKind::Absolute => source,
                    GaugeHandleKind::Relative => source + prev,
                };
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
