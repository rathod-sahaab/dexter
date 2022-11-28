use alloc::{borrow::ToOwned, boxed::Box, fmt::format};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier};
use esp32_hal::{
    pac::{Peripherals, RNG},
    Rng,
};
use rand_core::{impls, CryptoRng, RngCore};

use super::hasher::Hasher;

struct Generator {
    rng_core: Rng,
}

impl Generator {
    fn new() -> Self {
        let hardware_rng: RNG = Peripherals::take().unwrap().RNG;

        Self {
            rng_core: Rng::new(hardware_rng),
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
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl CryptoRng for Generator {}

pub struct ArgonHasher {
    generator: Generator,
}

impl ArgonHasher {
    fn new() -> Self {
        Self {
            generator: Generator::new(),
        }
    }
}

impl Hasher for ArgonHasher {
    fn hash(
        &self,
        bytes: &crate::dexter_core::common::Password,
    ) -> Box<crate::dexter_core::common::Hash> {
        let salt = SaltString::generate(self.generator);
        let argon = Argon2::default();

        let password_hash = argon.hash_password(bytes, &salt)?;
        Box::new(format(password_hash).as_bytes().to_owned())
    }

    fn verify(
        &self,
        hash: &crate::dexter_core::common::Hash,
        bytes: &crate::dexter_core::common::Password,
    ) -> bool {
        let argon = Argon2::default();
        argon.verify_password(bytes, hash).is_ok()
    }
}
