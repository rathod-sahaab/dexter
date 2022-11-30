use crate::dexter_core::common::{Hash, Password};

use super::hasher::Hasher;

#[derive(Default)]
pub struct NoHasher {}

impl Hasher for NoHasher {
    fn hash(&self, password: &Password) -> Hash {
        password.clone()
    }

    fn verify(&self, hash: &Hash, password: &Password) -> bool {
        hash.eq(password)
    }
}
