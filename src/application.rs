use esp32_hal::ehal::digital;

use crate::{
    dexter_core::{common::Password, Core},
    ui::contracts::InputUI,
};

pub enum ApplicationState {
    Locked,
    PasswordListening,
    Unlocked,
    PasswordBuilding,
}

pub struct Application<
    const DIGITS: usize,
    const KEYS: usize,
    I: InputUI<DIGITS, KEYS>,
    C: Core<DIGITS>,
> {
    state: ApplicationState,
    // static dispatch
    input: I,
    core: C,
}

fn from<const DIGITS: usize, const KEYS: usize>(data: [[bool; KEYS]; DIGITS]) -> Password<DIGITS> {
    data.map(|digit_keys| {
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

impl<const DIGITS: usize, const KEYS: usize, I: InputUI<DIGITS, KEYS>, C: Core<DIGITS>>
    Application<DIGITS, KEYS, I, C>
{
    fn password_reset_code(keys: [bool; KEYS]) -> bool {
        Self::only_one_key(keys, KEYS - 1)
    }

    fn lock_code(keys: [bool; KEYS]) -> bool {
        Self::only_one_key(keys, KEYS - 1)
    }

    fn only_one_key(keys: [bool; KEYS], allowed_key_index: usize) -> bool {
        keys.into_iter()
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
                if let Some(password) = self.input.digits_input() {
                    if self.core.verify_password(&from(password)) {
                        self.state = ApplicationState::Unlocked;
                    } else {
                        self.state = ApplicationState::Locked;
                    }
                }
            }
            ApplicationState::Unlocked => {
                if let Some(keys) = self.input.keys_input() {
                    if Self::lock_code(keys) {
                        self.state = ApplicationState::Locked
                    } else if Self::password_reset_code(keys) {
                        self.state = ApplicationState::PasswordBuilding
                    }
                }
            }
            ApplicationState::PasswordBuilding => {
                if let Some(password) = self.input.digits_input() {
                    self.core.set_password(&from(password));
                } else {
                }
            }
        }
    }
}
