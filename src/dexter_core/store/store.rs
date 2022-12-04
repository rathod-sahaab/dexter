use crate::dexter_core::common::Hash;

pub trait HashStore<const HASH_LENGTH: usize> {
    fn set(&self, array: &Hash<HASH_LENGTH>);
    fn get(&self) -> Option<Hash<HASH_LENGTH>>;
}
