use super::{
    application_state::ApplicationState,
    core::DexterCore,
    traits::ui::{InputUI, KeysValue},
};

pub struct Application<
    const DIGITS: usize,
    const KEYS: usize,
    Input: InputUI<DIGITS, KEYS>,
    C: DexterCore<DIGITS>,
> {
    state: ApplicationState,
    input: Input,
    core: C,
}

impl<const DIGITS: usize, const KEYS: usize, I: InputUI<DIGITS, KEYS>, C: DexterCore<DIGITS>>
    Application<DIGITS, KEYS, I, C>
{
    fn password_reset_code(keys: &KeysValue<KEYS>) -> bool {
        Self::only_one_key(keys, KEYS - 1)
    }

    fn lock_code(keys: &KeysValue<KEYS>) -> bool {
        Self::only_one_key(keys, KEYS - 1)
    }

    fn only_one_key(keys: &KeysValue<KEYS>, allowed_key_index: usize) -> bool {
        keys.0
            .into_iter()
            .enumerate()
            .all(|(index, key)| (index == allowed_key_index) ^ key)
    }

    pub fn looper(&mut self) {
        match self.state {
            ApplicationState::Locked => {
                if let Some(keys) = self.input.keys_input() {
                    self.state = ApplicationState::PasswordListening;
                }
            }
            ApplicationState::PasswordListening => {
                if let Some(password_digits) = self.input.digits_input() {
                    if self.core.verify_password(&password_digits.into()) {
                        self.state = ApplicationState::Unlocked;
                    } else {
                        self.state = ApplicationState::Locked;
                    }
                }
            }
            ApplicationState::Unlocked => {
                if let Some(keys) = self.input.keys_input() {
                    if Self::lock_code(&keys) {
                        self.state = ApplicationState::Locked
                    } else if Self::password_reset_code(&keys) {
                        self.state = ApplicationState::PasswordBuilding
                    }
                }
            }
            ApplicationState::PasswordBuilding => {
                if let Some(password_digits) = self.input.digits_input() {
                    self.core.set_password(&password_digits.into());
                } else {
                }
            }
        }
    }
}
