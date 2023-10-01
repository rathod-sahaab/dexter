pub struct DefaultCore<
    H: Hasher<DIGITS, HASH_LENGTH>,
    S: store::HashStore<HASH_LENGTH>,
    const DIGITS: usize,
    const HASH_LENGTH: usize,
> {
    password_hash: Hash<HASH_LENGTH>,

    hasher: H,
    hash_store: S,
}

impl<
        H: Hasher<DIGITS, HASH_LENGTH>,
        S: store::HashStore<HASH_LENGTH>,
        const DIGITS: usize,
        const HASH_LENGTH: usize,
    > DefaultCore<H, S, DIGITS, HASH_LENGTH>
{
    pub fn new(hasher: H, hash_store: S, default_password: Password<DIGITS>) -> Self {
        let default_password_hash = hasher.hash(&default_password);
        let password_hash = match hash_store.get() {
            Some(ph) => ph,
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
        H: Hasher<DIGITS, HASH_LENGTH>,
        S: store::HashStore<HASH_LENGTH>,
        const DIGITS: usize,
        const HASH_LENGTH: usize,
    > DexterCore<DIGITS> for DefaultCore<H, S, DIGITS, HASH_LENGTH>
{
    fn verify_password(&self, password: &Password<DIGITS>) -> bool {
        self.hasher.verify(&self.password_hash, password)
    }

    fn set_password(&mut self, new_password: &Password<DIGITS>) -> bool {
        self.password_hash = self.hasher.hash(new_password);

        self.hash_store.set(&self.password_hash);
        true
    }
}
