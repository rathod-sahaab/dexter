pub mod devices;
pub mod stateful_devices;

pub type DigitKeysValue<const DIGITS: usize, const KEYS: usize> = [[bool; KEYS]; DIGITS];
pub type KeysValue<const KEYS: usize> = [bool; KEYS];

pub trait InputUI<const DIGITS: usize, const KEYS: usize> {
    fn digits_input(&self) -> Option<DigitKeysValue<DIGITS, KEYS>>;
    fn keys_input(&self) -> Option<KeysValue<KEYS>>;
}
