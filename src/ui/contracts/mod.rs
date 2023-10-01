pub mod devices;
pub mod stateful_devices;

pub trait InputUI<const DIGITS: usize, const KEYS: usize> {
    fn digits_input(&self) -> Option<[[bool; KEYS]; DIGITS]>;
    fn keys_input(&self) -> Option<[bool; KEYS]>;
}
