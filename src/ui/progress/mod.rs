use crate::commons::bounds::Bounds;
mod gpio_progress_bar;

pub trait Progress {
    fn show(&mut self, bounds: Bounds<usize>, current: usize);
}

