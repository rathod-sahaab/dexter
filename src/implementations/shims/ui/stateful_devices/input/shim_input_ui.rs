use crate::ui::contracts::InputUI;

struct ShimInputUI<const DIGITS: usize, const KEYS: usize> {}

impl<const DIGITS: usize, const KEYS: usize> InputUI<DIGITS, KEYS> for ShimInputUI<DIGITS, KEYS> {
    fn digits_input(&self) -> Option<[[bool; KEYS]; DIGITS]> {
        Some([[false; KEYS]; DIGITS])
    }

    fn keys_input(&self) -> Option<[bool; KEYS]> {
        Some([false; KEYS])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_all_false() {
        let shim_ui = ShimInputUI::<5, 4> {};

        assert_eq!(shim_ui.keys_input(), Some([false; 4]));
    }
}
