use super::password::Password;

pub type Hash<const LENGTH: usize> = [u8; LENGTH];

pub trait Hasher<const DIGITS: usize, const HASH_LENGTH: usize> {
    // Hash bytes, produce one-way encrypted constant length hash.
    fn hash(&self, password: &Password<DIGITS>) -> Hash<HASH_LENGTH>;

    // Verify if
    fn verify(&self, hash: &Hash<HASH_LENGTH>, password: &Password<DIGITS>) -> bool;
}
