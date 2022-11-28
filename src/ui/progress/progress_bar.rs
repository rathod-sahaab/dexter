use crate::commons::bounds::Bounds;

pub trait Progress {
    fn show(&mut self, bounds: Bounds<usize>, current: usize);
}
