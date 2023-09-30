pub enum SFIState {
    Success,
    Failure,
    Off,
}
pub trait SuccessFailureIndicator {
    fn set(&mut self, state: SFIState);
}
