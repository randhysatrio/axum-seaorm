use validator::Validate;

use crate::errors::APIResult;

pub mod auth;
pub mod brand;
pub mod cart;
pub mod category;
pub mod product;

pub fn validate_payload<T: Validate>(payload: &T) -> APIResult<()> {
    Ok(payload.validate()?)
}
