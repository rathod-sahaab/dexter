use crate::dexter_core::traits::ui::KeysValue;

pub trait Keypad<const KEYS: usize> {
    fn get(&self) -> KeysValue<KEYS>;
}
