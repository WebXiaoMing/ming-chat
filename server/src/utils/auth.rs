use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::error::AppError;

#[allow(dead_code)]
pub fn verify_password(client_hash: &str, db_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(client_hash)?;
    let is_ok = Argon2::default()
        .verify_password(db_hash.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_ok)
}

// 密码哈希
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}
