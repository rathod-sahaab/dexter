use crate::traits::ui::KeysValue;
use core::option::Option;

/**
 * This is a buffered keypad only when the input is final do we return anything,
 * else we return none.
 *
 * [0]: 0 0 0 1 0 -> None
 * [1]: 1 0 0 1 0 -> None
 * [2]: 1 1 0 1 0 -> None
 * [3]: 0 0 0 0 0 -> Some(1 1 0 1 0)
 * [4]: 0 0 0 0 0 -> None
 *
 * Only on the [3] were we sure that user has completely entered the input value.
 *
 * This might vary from implementation to implementation.
 * This for example was for touch keys without a "flush" key.
 *
 * With first key/MSB as "flush" key
 *
 * [0]: 0 0 0 0 1 0 -> None
 * [1]: 1 1 0 0 1 0 -> Some(1 1 0 1 0)
 * [2]: 0 1 1 0 1 0 -> None
 * [3]: 0 0 0 0 0 0 -> None
 * [4]: 0 0 0 0 0 0 -> None
 */
pub trait StatefulKeypad<const KEYS: usize> {
    fn get(&self) -> Option<KeysValue<KEYS>>;
}
