use crate::commons::bounds::Bounds;

pub trait Progress {
    /**
     * Should behave as value was clamped.
     * When display of progress depends on provided minimum and max value.
     */
    fn show_bounded(&mut self, bounds: Bounds<usize>, current: usize);

    /**
     * Should behave as value was clamped.
     * When display depends on absolute values.
     */
    fn show(&mut self, current: usize);
}
