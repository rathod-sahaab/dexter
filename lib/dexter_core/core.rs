use super::traits::secrets::password::Password;

pub trait DexterCore<const DIGITS: usize> {
    fn verify_password(&self, password: &Password<DIGITS>) -> bool;
    fn set_password(&mut self, new_password: &Password<DIGITS>) -> bool;
}
