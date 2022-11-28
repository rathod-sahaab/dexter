use esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin as OutputPin;

use crate::commons::bounds::Bounds;

use super::progress::Progress;

pub struct GpioProgressBar<T: OutputPin, const MAX_PINS: usize> {
    pins: [T; MAX_PINS],
}

impl<T: OutputPin, const MAX_PINS: usize> GpioProgressBar<T, MAX_PINS> {
    pub fn new(pins: [T; MAX_PINS]) -> Self {
        Self { pins }
    }
}

impl<T: OutputPin, const MAX_PINS: usize> Progress for GpioProgressBar<T, MAX_PINS> {
    fn show(&mut self, bounds: Bounds<usize>, current: usize) {
        let progress = (MAX_PINS * (current - bounds.min)) / (bounds.max - bounds.min);

        (0..MAX_PINS).for_each(|i| {
            if i < progress {
                self.pins[i].set_high().unwrap_or(());
            } else {
                self.pins[i].set_low().unwrap_or(());
            }
        })
    }
}
