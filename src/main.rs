#![no_std]
#![no_main]

mod commons;
mod dexter_core;
mod ui;

use core::convert::Infallible;

use argon2::Error;
use dexter_core::hasher::no_hasher::NoHasher;

extern crate alloc;

use esp32_hal::{
    clock::ClockControl,
    gpio::IO,
    gpio_types::OutputPin,
    pac::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay,
    // Delay,
    // Serial,
    Rtc,
};

use esp_backtrace as _;

use crate::dexter_core::{store::no_store::NoStore, DefaultCore};
#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 128 * 1024;

    extern "C" {
        static mut _heap_start: u32;
        static mut _heap_end: u32;
    }

    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        let heap_end = &_heap_end as *const _ as usize;
        assert!(
            heap_end - heap_start > HEAP_SIZE,
            "Not enough available heap memory."
        );
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

#[xtensa_lx_rt::entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // let serial0 = Serial::new(peripherals.UART0);

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    let delay = Delay::new(&clocks);

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    execute(io, delay)
}

fn execute(io: IO, delay: Delay) -> ! {
    const DIGITS: usize = 5;
    const KEYS: usize = 4;

    // ------- Core -------
    let hasher = NoHasher::new();
    let store = NoStore::new();

    let core = DefaultCore::new(hasher, store, [3; DIGITS]);

    loop {
        // ui.cycle()
    }
}
