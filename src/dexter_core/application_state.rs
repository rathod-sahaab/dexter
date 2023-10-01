// TODO: use State machine
pub enum ApplicationState {
    Locked,
    PasswordListening,
    Unlocked,
    PasswordBuilding,
}

pub enum ApplicationEvents {
    Activated,
    WrongPassword,
    CorrectPassword,
    ManuallyLocked,
    PasswordResetSelected,
    PasswordReset,
}
