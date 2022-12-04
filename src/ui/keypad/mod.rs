pub mod gpio_keypad;

pub trait Keypad<const KEYS: usize> {
    fn read(&self) -> [bool; KEYS];
}
