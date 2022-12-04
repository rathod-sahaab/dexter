use crate::dexter_core::{common::Renderable, Core};

use self::{
    keypad::Keypad, progress::ProgressBar, success_failure_indicator::SuccessFailureIndicator,
};

pub mod keypad;
pub mod progress;
pub mod success_failure_indicator;

pub enum UiState {
    Locked,
    Open,
    PasswordChange,
}

// TODO: Add state to account for PASSWORD_RESET, LOCKED, OPEN
pub struct UI<
    P: ProgressBar,
    I: SuccessFailureIndicator + Renderable,
    K: Keypad<KEYS>,
    const KEYS: usize,
    const DIGITS: usize,
    C: Core<DIGITS>,
> {
    // Dependencies
    progress: P,
    sfi: I,
    keypad: K,
    core: C,
    // TODO: add bolt/shackle part witch opens and closes

    // Members
    state: UiState,
    cursor: usize,
    prev_key_presses: [bool; KEYS],
    prev_key_presses_true_count: usize,

    combination: [u8; DIGITS],
}

impl<
        P: ProgressBar,
        I: SuccessFailureIndicator + Renderable,
        K: Keypad<KEYS>,
        const KEYS: usize,
        const DIGITS: usize,
        C: Core<DIGITS>,
    > UI<P, I, K, KEYS, DIGITS, C>
{
    pub fn new(progress: P, sfi: I, keypad: K, core: C) -> Self {
        Self {
            // dependencies
            progress,
            sfi,
            keypad,
            core,

            // members
            state: UiState::Locked,
            cursor: 0,
            prev_key_presses: [false; KEYS],
            prev_key_presses_true_count: 0,
            combination: [0; DIGITS],
        }
    }

    fn reset(&mut self) {
        self.cursor = 0;
        self.prev_key_presses = [false; KEYS];
        self.combination = [0; DIGITS];
        self.prev_key_presses_true_count = 0;
    }

    fn set_state(&mut self, state: UiState) {
        match state {
            UiState::Locked => {
                self.state = UiState::Locked;
            }
            UiState::Open => {
                self.state = UiState::Open;
            }
            UiState::PasswordChange => {
                self.state = UiState::PasswordChange;
            }
        }
    }

    pub fn cycle(&mut self) {
        match self.state {
            UiState::Locked => self.locked_cycle(),
            UiState::Open => self.open_cycle(),
            UiState::PasswordChange => self.password_change_cycle(),
        }
    }

    fn locked_cycle(&mut self) {
        // input
        let key_presses = self.keypad.read();

        // compute
        let true_count = count_trues(&key_presses);

        if true_count == 0 {
            if self.prev_key_presses_true_count != 0 {
                // Registering last key entry
                self.combination[self.cursor] = binary_to_num::<KEYS>(&self.prev_key_presses);

                // Clearing noted key press
                self.prev_key_presses_true_count = 0;
                self.prev_key_presses = [false; KEYS];

                // Updating stage
                self.cursor += 1;

                // passing to core
                if self.cursor == DIGITS {
                    if self.core.verify_password(&self.combination) {
                        self.sfi.set_success(true);
                        self.set_state(UiState::Open)
                    } else {
                        self.sfi.set_success(false);
                        self.set_state(UiState::Locked);
                        // TODO: Reset state to listen for password again
                    }
                    self.sfi.set_visible(true);
                }
            }
        } else if true_count >= self.prev_key_presses_true_count {
            self.prev_key_presses_true_count = true_count;
            self.prev_key_presses = key_presses;
        }

        // renders
        self.sfi.render();
        self.progress.show(self.cursor)
    }

    fn open_cycle(&mut self) {
        // input
        let key_presses = self.keypad.read();

        // compute
        let true_count = count_trues(&key_presses);

        if true_count == 0 {
            if self.prev_key_presses_true_count != 0 {
                if let Some(true) = self.prev_key_presses.first() {
                    self.set_state(UiState::Locked);
                } else if self.prev_key_presses_true_count == DIGITS {
                    self.set_state(UiState::PasswordChange);
                }

                // Clearing noted key press
                self.prev_key_presses_true_count = 0;
                self.prev_key_presses = [false; KEYS];
            }
        } else if true_count >= self.prev_key_presses_true_count {
            self.prev_key_presses_true_count = true_count;
            self.prev_key_presses = key_presses;
        }

        // renders
        self.sfi.render();
        self.progress.show(self.cursor)
    }

    fn password_change_cycle(&mut self) {
        // input
        let key_presses = self.keypad.read();

        // compute
        let true_count = count_trues(&key_presses);

        if true_count == 0 {
            if self.prev_key_presses_true_count != 0 {
                // Registering last key entry
                self.combination[self.cursor] = binary_to_num::<KEYS>(&self.prev_key_presses);

                if self.prev_key_presses_true_count == DIGITS {
                    // Cancel password change
                    self.set_state(UiState::Open);
                }

                // Clearing noted key press
                self.prev_key_presses_true_count = 0;
                self.prev_key_presses = [false; KEYS];

                // Updating stage
                self.cursor += 1;

                // passing to core
                if self.cursor == DIGITS {
                    if self.core.set_password(&self.combination) {
                        self.sfi.set_success(true);
                        self.set_state(UiState::Locked);
                    } else {
                        self.sfi.set_success(false);
                        self.reset();
                        self.set_state(UiState::PasswordChange);
                    }
                    self.sfi.set_visible(true);
                }
            }
        } else if true_count >= self.prev_key_presses_true_count {
            self.prev_key_presses_true_count = true_count;
            self.prev_key_presses = key_presses;
        }

        // renders
        self.sfi.render();
        self.progress.show(self.cursor)
    }
}

fn count_trues<const LENGTH: usize>(arr: &[bool; LENGTH]) -> usize {
    arr.iter().filter(|key_press| **key_press).count()
}

fn binary_to_num<const LENGTH: usize>(arr: &[bool; LENGTH]) -> u8 {
    arr.iter().enumerate().fold(0u8, |accum: u8, (i, val)| {
        accum
            + match val {
                true => 2u8.pow(i.try_into().unwrap()),
                false => 0,
            }
    })
}
