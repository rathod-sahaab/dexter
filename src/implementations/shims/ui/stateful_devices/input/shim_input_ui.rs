use dexter_core::traits::ui::{DigitKeysValue, InputUI, KeysValue};

struct ShimInputUI<const DIGITS: usize, const KEYS: usize> {}

impl<const DIGITS: usize, const KEYS: usize> InputUI<DIGITS, KEYS> for ShimInputUI<DIGITS, KEYS> {
    fn digits_input(&self) -> Option<DigitKeysValue<DIGITS, KEYS>> {
        Some(DigitKeysValue([[false; KEYS]; DIGITS]))
    }

    fn keys_input(&self) -> Option<KeysValue<KEYS>> {
        Some(KeysValue([false; KEYS]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_all_false() {
        let shim_ui = ShimInputUI::<5, 4> {};
        let result = shim_ui.keys_input();

        assert!(result.is_some());
    }
}
