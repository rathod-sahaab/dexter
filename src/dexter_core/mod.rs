mod common;
mod hash;
mod store;

use common::Password;

pub struct Core<H: hash::Hasher, HS: store::HashStore> {
    max_password_size: u32,
    password_hash: common::Hash,

    hasher: H,
    hash_store: HS,
}

impl<H, HS> Core<H, HS>
where
    H: hash::Hasher,
    HS: store::HashStore,
{
    pub fn new(max_password_size: u32, hasher: H, hash_store: HS) -> Self {
        let password_hash = hash_store.get();

        Core {
            max_password_size,
            hasher,
            hash_store,
            password_hash,
        }
    }

    pub fn get_password_length(&self) -> u32 {
        self.max_password_size
    }

    pub fn verify_password(&self, password: &Password) -> bool {
        self.hasher.verify(&self.password_hash, password)
    }

    pub fn set_password(&mut self, old_password: &Password, new_password: &Password) -> bool {
        if !self.verify_password(old_password) {
            return false;
        }

        self.password_hash = self.hasher.hash(new_password);

        self.hash_store.set(&self.password_hash);
        true
    }
}
