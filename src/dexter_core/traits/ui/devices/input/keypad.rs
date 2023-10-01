pub trait Keypad<const KEYS: usize> {
    fn get(&self) -> [bool; KEYS];
}
