use crate::dexter_core::common::{Hash, Password};

// Trait for a hasher, which is used to encrypt and verify bytes, generally for passwords.
pub trait Hasher {
    // Hash bytes, produce one-way encrypted constant length hash.
    fn hash(&self, password: &Password) -> Hash;

    // Verify if
    fn verify(&self, hash: &Hash, password: &Password) -> bool;
}
