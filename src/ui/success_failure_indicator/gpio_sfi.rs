use core::convert::Infallible;

use esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin as OutputPin;

use super::sfi::SuccessFailureIndicator;

pub struct GpioSuccessFailureIndicator<
    T: OutputPin<Error = Infallible>,
    U: OutputPin<Error = Infallible>,
> {
    success_pin: T,
    Failure_pin: U,
}

impl<T: OutputPin<Error = Infallible>, U: OutputPin<Error = Infallible>> SuccessFailureIndicator
    for GpioSuccessFailureIndicator<T, U>
{
    fn show(&mut self, show: bool, success: bool) {
        if !show {
            self.success_pin.set_low().unwrap();
            return;
        }

        if success {
            self.success_pin.set_high().unwrap();
            return;
        }

        self.Failure_pin.set_high().unwrap();
    }
}
