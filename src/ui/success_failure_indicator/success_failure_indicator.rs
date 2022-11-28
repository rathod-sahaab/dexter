pub trait SuccessFaliureIndicator {
    fn show(&mut self, show: bool, success: bool);
}
