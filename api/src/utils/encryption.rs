use bcrypt::DEFAULT_COST;
use tokio::sync::oneshot;

use crate::errors::{APIResult, AppError};

pub async fn hash_password(password: String) -> APIResult<String> {
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let hash = bcrypt::hash(password, DEFAULT_COST);

        let _ = tx.send(hash);
    });

    // The .await? will propagate the Tokio's RecvError, then we mapped the BcryptError into AppError;
    rx.await?.map_err(AppError::BcryptError)
}

pub async fn validate_password(password_input: String, password_to_cmp: String) -> APIResult<bool> {
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let validity = bcrypt::verify(password_input, &password_to_cmp);

        let _ = tx.send(validity);
    });

    rx.await?.map_err(AppError::BcryptError)
}
