pub trait Keypad<const KEYS: usize> {
    fn get() -> [bool; KEYS];
}
