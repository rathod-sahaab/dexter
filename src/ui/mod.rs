use crate::dexter_core::Core;

use self::{
    keypad::keypad::Keypad, progress::progress_bar::Progress,
    success_failure_indicator::sfi::SuccessFailureIndicator,
};

pub mod keypad;
pub mod progress;
pub mod success_failure_indicator;

struct UI<
    P: Progress,
    I: SuccessFailureIndicator,
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

    // Members
    cursor: usize,
    prev_key_presses: [bool; KEYS],
    prev_key_presses_true_count: usize,

    combination: [u8; DIGITS],
}

impl<
        P: Progress,
        I: SuccessFailureIndicator,
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
        self.prev_key_presses_true_count = 0;
    }

    pub fn cycle(&mut self) {
        let key_presses = self.keypad.read();

        let true_count = count_trues(&key_presses);

        if true_count == 0 && self.prev_key_presses_true_count != 0 {
            // set prev data
            self.prev_key_presses_true_count = 0;
            self.prev_key_presses = [false; KEYS];

            // Digit entered
            self.cursor += 1;

            self.progress.show(self.cursor);

            if self.cursor == DIGITS {
                self.core.verify_password(&self.combination);
            }

            if self.cursor >= DIGITS {
                self.reset();
                return;
            }
        } else if self.prev_key_presses_true_count <= true_count {
            self.prev_key_presses = key_presses;
            self.prev_key_presses_true_count = true_count;
        }
    }
}

fn count_trues<const LENGTH: usize>(arr: &[bool; LENGTH]) -> usize {
    arr.iter().filter(|key_press| **key_press).count()
}
