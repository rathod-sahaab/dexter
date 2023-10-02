use core::{convert::From, iter::IntoIterator, iter::Iterator, option::Option};

use super::secrets::password::Password;

pub mod devices;
pub mod stateful_devices;

pub struct DigitKeysValue<const DIGITS: usize, const KEYS: usize>(pub [[bool; KEYS]; DIGITS]);

pub struct KeysValue<const KEYS: usize>(pub [bool; KEYS]);

impl<const DIGITS: usize, const KEYS: usize> From<DigitKeysValue<DIGITS, KEYS>>
    for Password<DIGITS>
{
    fn from(value: DigitKeysValue<DIGITS, KEYS>) -> Self {
        value.0.map(|digit_keys| {
            digit_keys
                .into_iter()
                .rev()
                .enumerate()
                .fold(0u8, |acc, (index, value)| {
                    acc + if value {
                        (index as u8) * (index as u8)
                    } else {
                        0
                    }
                })
        })
    }
}

pub trait InputUI<const DIGITS: usize, const KEYS: usize> {
    fn digits_input(&self) -> Option<DigitKeysValue<DIGITS, KEYS>>;
    fn keys_input(&self) -> Option<KeysValue<KEYS>>;
}
