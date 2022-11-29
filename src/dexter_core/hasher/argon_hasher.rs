use crate::dexter_core::common::{Hash, Password};
use alloc::string::ToString;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use esp32_hal::{pac::RNG, Rng};
use rand_core::{impls, CryptoRng, RngCore};

use super::hasher::Hasher;

struct Generator {
    rng_core: Rng,
}

impl Generator {
    fn new(rng: RNG) -> Self {
        Self {
            rng_core: Rng::new(rng),
        }
    }
}

impl RngCore for Generator {
    fn next_u32(&mut self) -> u32 {
        self.rng_core.random()
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for Generator {}

pub struct ArgonHasher {
    salt: SaltString,
}

impl ArgonHasher {
    pub fn new(rng: RNG) -> Self {
        Self {
            salt: SaltString::generate(Generator::new(rng)),
        }
    }
}

impl ArgonHasher {}

impl Hasher for ArgonHasher {
    fn hash(&self, password: &Password) -> Hash {
        let argon = Argon2::default();

        let password_hash = argon
            .hash_password(password.as_bytes(), &self.salt)
            .unwrap();

        password_hash.to_string()
    }

    fn verify(&self, hash: &Hash, password: &Password) -> bool {
        let argon = Argon2::default();

        let passhash = PasswordHash::new(hash).unwrap();
        argon
            .verify_password(password.as_bytes(), &passhash)
            .is_ok()
    }
}
