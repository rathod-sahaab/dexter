pub mod gpio_sfi;

pub trait SuccessFailureIndicator {
    /**
     * This works like a latch i.e. remembers it's previous value.
     */
    fn set_visible(&mut self, visible: bool);
    fn set_success(&mut self, success: bool);
}
