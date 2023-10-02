use dexter_core::traits::secrets::hasher::Hash;
use dexter_core::traits::store::HashStore;

pub struct NoStore {}

impl<const HASH_LENGTH: usize> HashStore<HASH_LENGTH> for NoStore {
    fn set(&self, _: &Hash<HASH_LENGTH>) {}

    fn get(&self) -> Option<Hash<HASH_LENGTH>> {
        Some([3u8; HASH_LENGTH])
    }
}
