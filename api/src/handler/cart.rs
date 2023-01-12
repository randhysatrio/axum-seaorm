use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    errors::APIResponse,
    extractor::{body_extractor, query_extractor, ReqBody, ReqQuery},
    middlewares::CurrentUser,
    services::{CartData, CartService},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateOrUpdateCartRequest {
    product_id: i32,
    quantity: i32,
}
#[derive(Debug, Serialize)]
pub struct CreateOrUpdateCartResponse {
    success: bool,
    message: &'static str,
}
pub async fn create_or_update_cart(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    body: ReqBody<CreateOrUpdateCartRequest>,
) -> APIResponse<(StatusCode, Json<CreateOrUpdateCartResponse>)> {
    let CreateOrUpdateCartRequest {
        product_id,
        quantity,
    } = body_extractor(body)?;
    let db = &state.conn;

    let update_or_create_cart =
        CartService::create_or_update(db, current_user.id, product_id, quantity).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateOrUpdateCartResponse {
            success: true,
            message: update_or_create_cart,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct FindCartQuery {
    page: Option<i32>,
    size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct FindCartResponse {
    success: bool,
    total_page: u64,
    total_items: u64,
    data: Vec<CartData>,
}
pub async fn find_carts(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    query: ReqQuery<FindCartQuery>,
) -> APIResponse<(StatusCode, Json<FindCartResponse>)> {
    let query = query_extractor(query)?;
    let FindCartQuery { page, size } = query;

    let db = &state.conn;

    let (data, total_items, total_page) = CartService::get(db, current_user.id, page, size).await?;

    Ok((
        StatusCode::OK,
        Json(FindCartResponse {
            success: true,
            total_page,
            total_items,
            data,
        }),
    ))
}
