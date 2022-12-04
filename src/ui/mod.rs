use crate::dexter_core::{common::Renderable, Core};

use self::{
    keypad::keypad::Keypad, progress::progress_bar::Progress,
    success_failure_indicator::sfi::SuccessFailureIndicator,
};

pub mod keypad;
pub mod progress;
pub mod success_failure_indicator;

// TODO: Add state to account for PASSWORD_RESET, LOCKED, OPEN
struct UI<
    P: Progress,
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
    cursor: usize,
    prev_key_presses: [bool; KEYS],
    prev_key_presses_true_count: usize,

    combination: [u8; DIGITS],
}

impl<
        P: Progress,
        I: SuccessFailureIndicator + Renderable,
        K: Keypad<KEYS>,
        const KEYS: usize,
        const DIGITS: usize,
        C: Core<DIGITS>,
    > UI<P, I, K, KEYS, DIGITS, C>
{
    pub fn new(progress: P, sfi: I, keypad: K, core: C) -> Self {
        Self {
            progress,
            sfi,
            keypad,
            core,
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

    pub fn cycle(&mut self) {
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
                    } else {
                        self.sfi.set_success(false);
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
