mod auth_service;
mod brand_service;
mod cart_service;
mod category_service;
mod product_service;

pub use auth_service::AuthService;
pub use brand_service::BrandService;
pub use cart_service::{CartData, CartService};
pub use category_service::CategoryService;
pub use product_service::{ProductData, ProductService};

use crate::errors::{APIResult, AppError};

pub fn page_matcher(page: Option<i32>) -> APIResult<u64> {
    match page {
        Some(p) => {
            if p <= 0 {
                Err(AppError::InvalidPage)
            } else {
                Ok((p - 1) as u64)
            }
        }
        None => Ok(0),
    }
}

pub fn size_matcher(size: Option<i32>) -> APIResult<u64> {
    match size {
        Some(s) => {
            if s <= 0 {
                Err(AppError::InvalidSize)
            } else {
                Ok((s) as u64)
            }
        }
        None => Ok(10),
    }
}
