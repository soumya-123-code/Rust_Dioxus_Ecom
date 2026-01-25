use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn verify(password: &str, hash: &str) -> bool {
    // Try Argon2 first
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        return Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
    }
    
    // TODO: Add support for Bcrypt (Laravel default) if needed
    // This would require adding the 'bcrypt' crate
    
    false
}

pub fn hash(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2.hash_password(password.as_bytes(), &salt)
        .map(|t| t.to_string())
        .map_err(|e| e.to_string())
}
