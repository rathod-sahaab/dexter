pub type Hash<const LENGTH: usize> = [u8; LENGTH];
pub type Password<const DIGITS: usize> = [u8; DIGITS];

pub trait Renderable {
    fn render(&mut self);
}
