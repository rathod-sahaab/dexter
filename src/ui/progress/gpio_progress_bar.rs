use core::convert::Infallible;

use esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin as OutputPin;

use crate::commons::bounds::Bounds;

use super::ProgressBar;

pub struct GpioProgressBar<'a, const MAX_PINS: usize> {
    pins: [&'a mut dyn OutputPin<Error = Infallible>; MAX_PINS],
}

impl<'a, const MAX_PINS: usize> GpioProgressBar<'a, MAX_PINS> {
    pub fn new(pins: [&'a mut dyn OutputPin<Error = Infallible>; MAX_PINS]) -> Self {
        Self { pins }
    }
}

impl<'a, const MAX_PINS: usize> ProgressBar for GpioProgressBar<'a, MAX_PINS> {
    fn show_bounded(&mut self, bounds: Bounds<usize>, current: usize) {
        let progress = (MAX_PINS * (current - bounds.min)) / (bounds.max - bounds.min);

        for (i, pin) in self.pins.iter_mut().enumerate() {
            if i < progress {
                pin.set_high().unwrap();
            } else {
                pin.set_low().unwrap();
            }
        }
    }

    fn show(&mut self, current: usize) {
        for (i, pin) in self.pins.iter_mut().enumerate() {
            if i < current {
                pin.set_high().unwrap();
            } else {
                pin.set_low().unwrap();
            }
        }
    }
}
