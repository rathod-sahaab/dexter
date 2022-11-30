#![no_std]
#![no_main]

mod commons;
mod dexter_core;
mod ui;

use alloc::string::String;
use commons::{bounds::Bounds, logger::Logger};

use alloc::format;

use dexter_core::hasher::{argon_hasher::ArgonHasher, hasher::Hasher, no_hasher::NoHasher};
use ui::progress::{gpio_progress_bar::GpioProgressBar, progress_bar::Progress};

extern crate alloc;

use core::fmt::Write;
use esp32_hal::{
    clock::ClockControl, gpio::IO, pac::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
    Serial,
};

use esp_backtrace as _;
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
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Real code begins
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut led = io.pins.gpio25.into_push_pull_output();
    let mut led2 = io.pins.gpio26.into_push_pull_output();

    let mut progress_bar = GpioProgressBar::new([&mut led, &mut led2]);

    let serial0 = Serial::new(peripherals.UART0);

    let mut logger = Logger::new(serial0, &ALLOCATOR);

    logger.logln("------------------ Started Program ------------------");

    init_heap();

    logger.logln("------------------ INIT HEAP ------------------");

    let mut delay = Delay::new(&clocks);

    let no_hasher = NoHasher::default();

    let pass = String::from("Abhay");

    logger.logln("------------------ Allocated String ------------------");

    let hash = no_hasher.hash(&pass);

    logger.logln(format!("Hash: {}", hash.as_str()).as_str());

    let mut prog = 0;
    loop {
        progress_bar.show(Bounds { max: 2, min: 0 }, prog);
        prog += 1;

        if prog > 2 {
            prog = 0;
        }

        delay.delay_ms(500u32);
    }
}
