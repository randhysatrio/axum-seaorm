use validator::Validate;

use crate::errors::APIResult;

pub mod auth;

pub fn validate_payload<T: Validate>(payload: &T) -> APIResult<()> {
    payload.validate()?;

    Ok(())
}
