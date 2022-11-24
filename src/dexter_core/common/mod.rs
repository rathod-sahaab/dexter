pub const MAX_PASSWORD_HASH_LENGTH: usize = 1024;
pub const MAX_PASSWORD_LENGTH: usize = 100;

pub type Hash = [u8; MAX_PASSWORD_HASH_LENGTH];
pub type Password = [u8; MAX_PASSWORD_LENGTH];
