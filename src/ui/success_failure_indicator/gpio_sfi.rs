use core::convert::Infallible;

use esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin as OutputPin;

use super::sfi::SuccessFaliureIndicator;

pub struct GpioSuccessFaliureIndicator<
    T: OutputPin<Error = Infallible>,
    U: OutputPin<Error = Infallible>,
> {
    success_pin: T,
    faliure_pin: U,
}

impl<T: OutputPin<Error = Infallible>, U: OutputPin<Error = Infallible>> SuccessFaliureIndicator
    for GpioSuccessFaliureIndicator<T, U>
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

        self.faliure_pin.set_high().unwrap();
    }
}
