use super::common::Hash;

pub trait HashStore {
    fn set(&self, array: &Hash);
    fn get(&self) -> Option<&Hash>;
}
