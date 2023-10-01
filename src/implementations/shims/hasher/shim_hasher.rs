#[derive(Default)]
pub struct NoHasher {}

impl<const DIGITS: usize> Hasher<DIGITS, DIGITS> for NoHasher {
    fn hash(&self, password: &Password<DIGITS>) -> Hash<DIGITS> {
        *password
    }

    fn verify(&self, hash: &Hash<DIGITS>, password: &Password<DIGITS>) -> bool {
        hash.eq(password)
    }
}
