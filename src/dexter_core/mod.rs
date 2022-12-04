pub mod common;
pub mod hasher;
pub mod store;

use alloc::borrow::ToOwned;
use common::Password;

pub trait Core<const DIGITS: usize> {
    fn verify_password(&self, password: &Password<DIGITS>) -> bool;
    fn set_password(
        &mut self,
        old_password: &Password<DIGITS>,
        new_password: &Password<DIGITS>,
    ) -> bool;
}

pub struct DefaultCore<
    H: hasher::hasher::Hasher<DIGITS, HASH_LENGTH>,
    S: store::HashStore<HASH_LENGTH>,
    const DIGITS: usize,
    const HASH_LENGTH: usize,
> {
    password_hash: common::Hash<HASH_LENGTH>,

    hasher: H,
    hash_store: S,
}

impl<
        H: hasher::hasher::Hasher<DIGITS, HASH_LENGTH>,
        S: store::HashStore<HASH_LENGTH>,
        const DIGITS: usize,
        const HASH_LENGTH: usize,
    > DefaultCore<H, S, DIGITS, HASH_LENGTH>
{
    pub fn new(hasher: H, hash_store: S, default_password: Password<DIGITS>) -> Self {
        let default_password_hash = hasher.hash(&default_password);
        let password_hash = match hash_store.get() {
            Some(ph) => ph.to_owned(),
            None => default_password_hash,
        };

        Self {
            hasher,
            hash_store,
            password_hash,
        }
    }
}

impl<
        H: hasher::hasher::Hasher<DIGITS, HASH_LENGTH>,
        S: store::HashStore<HASH_LENGTH>,
        const DIGITS: usize,
        const HASH_LENGTH: usize,
    > Core<DIGITS> for DefaultCore<H, S, DIGITS, HASH_LENGTH>
{
    fn verify_password(&self, password: &Password<DIGITS>) -> bool {
        self.hasher.verify(&self.password_hash, password)
    }

    fn set_password(
        &mut self,
        old_password: &Password<DIGITS>,
        new_password: &Password<DIGITS>,
    ) -> bool {
        if !self.verify_password(old_password) {
            return false;
        }

        self.password_hash = self.hasher.hash(new_password);

        self.hash_store.set(&self.password_hash);
        true
    }
}
