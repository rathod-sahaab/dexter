use super::HashStore;

pub struct NoStore {}

impl NoStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const HASH_LENGTH: usize> HashStore<HASH_LENGTH> for NoStore {
    fn set(&self, _: &crate::dexter_core::common::Hash<HASH_LENGTH>) {}

    fn get(&self) -> Option<crate::dexter_core::common::Hash<HASH_LENGTH>> {
        Some([3u8; HASH_LENGTH])
    }
}
