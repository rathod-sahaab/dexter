pub mod common;
pub mod hasher;
pub mod store;

use alloc::borrow::ToOwned;
use common::{Hash, Password};

pub struct Core<H: hasher::hasher::Hasher, S: store::HashStore> {
    max_password_size: u32,
    password_hash: common::Hash,

    hasher: H,
    hash_store: S,
}

impl<H: hasher::hasher::Hasher, S: store::HashStore> Core<H, S> {
    pub fn new(
        max_password_size: u32,
        hasher: H,
        hash_store: S,
        default_password: Password,
    ) -> Self {
        let default_password_hash = hasher.hash(&default_password);
        let password_hash: Hash = match hash_store.get() {
            Some(ph) => ph.to_owned(),
            None => default_password_hash,
        };

        Self {
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
