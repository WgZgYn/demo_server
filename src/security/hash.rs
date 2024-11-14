use argon2::Config;
use rand::random;

const LEN: usize = 32;

pub fn gen_salt() -> [u8; LEN] {
    random()
}

pub fn password_hash(password: &str, salt: &[u8]) -> String {
    let cfg: Config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &cfg).unwrap()
}

pub fn password_verify(password_hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(password_hash, password).unwrap()
}
