use super::common::Hash;

pub trait HashStore<const DIGITS: usize> {
    fn set(&self, array: &Hash<DIGITS>);
    fn get(&self) -> Option<&Hash<DIGITS>>;
}
