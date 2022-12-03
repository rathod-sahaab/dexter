pub trait SuccessFailureIndicator {
    fn show(&mut self, show: bool, success: bool);
}
