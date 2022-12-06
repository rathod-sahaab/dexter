#![no_std]
#![no_main]

mod commons;
mod dexter_core;
mod ui;

use dexter_core::hasher::no_hasher::NoHasher;
use ui::{
    keypad::gpio_keypad::GpioKeypad, progress::gpio_progress_bar::GpioProgressBar,
    success_failure_indicator::gpio_sfi::GpioSuccessFailureIndicator, UI,
};

extern crate alloc;

use esp32_hal::{
    clock::ClockControl,
    gpio::IO,
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

    // Real code begins
    const DIGITS: usize = 5;
    const KEYS: usize = 4;

    // ------- Core -------
    let hasher = NoHasher::new();
    let store = NoStore::new();

    let core = DefaultCore::new(hasher, store, [3; DIGITS]);

    // -------  UI  -------
    // progress bar
    //  avoiding error[E0716]: temporary value dropped while borrowed

    let mut progress_led_0 = io.pins.gpio21.into_push_pull_output();
    let mut progress_led_1 = io.pins.gpio19.into_push_pull_output();
    let mut progress_led_2 = io.pins.gpio18.into_push_pull_output();
    let mut progress_led_3 = io.pins.gpio5.into_push_pull_output();
    let mut progress_led_4 = io.pins.gpio17.into_push_pull_output();

    let progress = GpioProgressBar::<DIGITS>::new([
        // order is critical
        &mut progress_led_0,
        &mut progress_led_1,
        &mut progress_led_2,
        &mut progress_led_3,
        &mut progress_led_4,
    ]);

    let sfi = GpioSuccessFailureIndicator::new(
        io.pins.gpio23.into_push_pull_output(),
        io.pins.gpio22.into_push_pull_output(),
        &delay,
    );

    let mut keypad_key_0 = io.pins.gpio27.into_pull_down_input();
    let mut keypad_key_1 = io.pins.gpio14.into_pull_down_input();
    let mut keypad_key_2 = io.pins.gpio12.into_pull_down_input();
    let mut keypad_key_3 = io.pins.gpio13.into_pull_down_input();

    let keypad = GpioKeypad::<KEYS>::new([
        // order or this array is critical
        &mut keypad_key_0,
        &mut keypad_key_1,
        &mut keypad_key_2,
        &mut keypad_key_3,
    ]);

    let mut ui = UI::new(progress, sfi, keypad, core);

    loop {
        ui.cycle()
    }
}
