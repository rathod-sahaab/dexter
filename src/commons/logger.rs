use core::fmt::Write;
use esp32_hal::{pac::UART0, Serial};
use esp_alloc::EspHeap;

pub struct Logger<'a> {
    serial: Serial<UART0>,
    alloc: &'a EspHeap,
}

impl<'a> Logger<'a> {
    pub fn new(serial: Serial<UART0>, alloc: &'a EspHeap) -> Self {
        Self { serial, alloc }
    }
    pub fn logln(&mut self, output: &str) {
        writeln!(self.serial, "{}", output).unwrap();
        writeln!(self.serial, "Heap used: {}", self.alloc.used()).unwrap();
    }
}
