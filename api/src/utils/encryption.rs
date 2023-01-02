use crate::dto::Error;

pub fn hash_password(password: String) -> String {
    bcrypt::hash(password, 14)
        .map_err(|e| Error::BcryptError(e))
        .unwrap()
}

pub fn validate_password(password_input: String, password_to_cmp: &str) -> bool {
    bcrypt::verify(password_input, password_to_cmp)
        .map_err(|e| Error::BcryptError(e))
        .unwrap()
}
