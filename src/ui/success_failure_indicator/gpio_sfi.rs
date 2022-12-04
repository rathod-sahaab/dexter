use core::convert::Infallible;

use esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin as OutputPin;

use crate::dexter_core::common::Renderable;

use super::sfi::SuccessFailureIndicator;

pub struct GpioSuccessFailureIndicator<
    T: OutputPin<Error = Infallible>,
    U: OutputPin<Error = Infallible>,
> {
    success_pin: T,
    faiure_pin: U,

    visible: bool,
    success: bool,
}

impl<T: OutputPin<Error = Infallible>, U: OutputPin<Error = Infallible>> Renderable
    for GpioSuccessFailureIndicator<T, U>
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
            self.faiure_pin.set_high().unwrap();
        } else {
            self.faiure_pin.set_low().unwrap();
        }
    }
}

impl<T: OutputPin<Error = Infallible>, U: OutputPin<Error = Infallible>> SuccessFailureIndicator
    for GpioSuccessFailureIndicator<T, U>
{
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn set_success(&mut self, success: bool) {
        self.success = success;
    }
}
