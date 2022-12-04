use core::convert::Infallible;

use esp32_hal::prelude::_embedded_hal_digital_v2_InputPin as InputPin;

use super::Keypad;

pub struct GpioKeypad<'a, const MAX_PINS: usize> {
    pins: [&'a dyn InputPin<Error = Infallible>; MAX_PINS],
}

impl<'a, const MAX_PINS: usize> GpioKeypad<'a, MAX_PINS> {
    pub fn new(pins: [&'a dyn InputPin<Error = Infallible>; MAX_PINS]) -> Self {
        Self { pins }
    }
}

impl<'a, const MAX_PINS: usize> Keypad<MAX_PINS> for GpioKeypad<'a, MAX_PINS> {
    fn read(&self) -> [bool; MAX_PINS] {
        self.pins.map(|pin| pin.is_high().unwrap())
    }
}
