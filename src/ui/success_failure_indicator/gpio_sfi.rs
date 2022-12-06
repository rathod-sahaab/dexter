use core::convert::Infallible;

use esp32_hal::{prelude::_embedded_hal_digital_v2_OutputPin as OutputPin, Delay};

use crate::dexter_core::common::Renderable;

use super::SuccessFailureIndicator;

pub struct GpioSuccessFailureIndicator<
    'a,
    T: OutputPin<Error = Infallible>,
    U: OutputPin<Error = Infallible>,
> {
    delay: &'a Delay,

    success_pin: T,
    failure_pin: U,

    visible: bool,
    success: bool,
}

impl<'a, T: OutputPin<Error = Infallible>, U: OutputPin<Error = Infallible>>
    GpioSuccessFailureIndicator<'a, T, U>
{
    pub fn new(success_pin: T, failure_pin: U, delay: &'a Delay) -> Self {
        Self {
            delay,
            success_pin,
            failure_pin,
            visible: false,
            success: false,
        }
    }
}

impl<'a, T: OutputPin<Error = Infallible>, U: OutputPin<Error = Infallible>> Renderable
    for GpioSuccessFailureIndicator<'a, T, U>
{
    fn render(&mut self) {
        let show_success = self.visible && self.success;
        let show_faliure = self.visible && !self.success;

        if show_success {
            self.success_pin.set_high().unwrap();
        } else {
            self.success_pin.set_low().unwrap();
        }

        if show_faliure {
            self.failure_pin.set_high().unwrap();
        } else {
            self.failure_pin.set_low().unwrap();
        }

        if show_faliure {
            self.delay.delay(3 * 1_000_000);
            self.visible = false;
        }
    }
}

impl<'a, T: OutputPin<Error = Infallible>, U: OutputPin<Error = Infallible>> SuccessFailureIndicator
    for GpioSuccessFailureIndicator<'a, T, U>
{
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn set_success(&mut self, success: bool) {
        self.success = success;
    }
}
