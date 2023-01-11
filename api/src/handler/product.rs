use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::validate_payload;
use crate::{
    errors::APIResponse,
    extractor::{body_extractor, path_extractor, query_extractor, ReqBody, ReqPath, ReqQuery},
    services::{product_service::ProductData, ProductService},
    AppState,
};

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(required(message = "Name is required"))]
    name: Option<String>,
    #[validate(required(message = "Price is required"))]
    price: Option<i32>,
    #[validate(required(message = "Stock is required"))]
    stock: Option<i32>,
    #[validate(required(message = "Category_id is required"))]
    category_id: Option<i32>,
    #[validate(required(message = "Brand_id is required"))]
    brand_id: Option<i32>,
    description: Option<String>,
}
pub async fn create_product(
    State(state): State<AppState>,
    Json(body): Json<CreateProductRequest>,
) -> APIResponse<(StatusCode, Json<ProductResponse>)> {
    validate_payload(&body)?;

    let db = &state.conn;

    let CreateProductRequest {
        name,
        price,
        stock,
        category_id,
        brand_id,
        description,
    } = body;

    let created_product = ProductService::create(
        db,
        name.unwrap(),
        price.unwrap(),
        stock.unwrap(),
        category_id.unwrap(),
        brand_id.unwrap(),
        description,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(ProductResponse {
            success: true,
            message: format!("Created product with id: {}", created_product.id),
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct FindProductParams {
    keyword: Option<String>,
    page: Option<i32>,
    size: Option<i32>,
    all: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct FindProductsResponse {
    success: bool,
    total_page: u64,
    total_items: u64,
    data: Vec<ProductData>,
}
pub async fn find_products(
    State(state): State<AppState>,
    query: ReqQuery<FindProductParams>,
) -> APIResponse<(StatusCode, Json<FindProductsResponse>)> {
    let query = query_extractor(query)?;

    let FindProductParams {
        keyword,
        page,
        size,
        all,
    } = query;

    let db = &state.conn;
    let (data, total_items, total_page) =
        ProductService::find(db, keyword, page, size, all).await?;

    Ok((
        StatusCode::OK,
        Json(FindProductsResponse {
            success: true,
            total_page,
            total_items,
            data,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductData {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub stock: Option<i32>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub brand_id: Option<i32>,
}

pub async fn update_product(
    State(state): State<AppState>,
    id: ReqPath<i32>,
    update_data: ReqBody<UpdateProductData>,
) -> APIResponse<(StatusCode, Json<ProductResponse>)> {
    let id = path_extractor(id)?;
    let update_data = body_extractor(update_data)?;
    let db = &state.conn;

    ProductService::update(db, id, update_data).await?;

    Ok((
        StatusCode::OK,
        Json(ProductResponse {
            success: true,
            message: "Product updated successfully".to_owned(),
        }),
    ))
}

pub async fn delete_product(
    State(state): State<AppState>,
    id: ReqPath<i32>,
) -> APIResponse<(StatusCode, Json<ProductResponse>)> {
    let id = path_extractor(id)?;
    let db = &state.conn;

    ProductService::delete(db, id).await?;

    Ok((
        StatusCode::OK,
        Json(ProductResponse {
            success: true,
            message: "Product deleted successfully".to_owned(),
        }),
    ))
}

pub async fn restore_product(
    State(state): State<AppState>,
    id: ReqPath<i32>,
) -> APIResponse<(StatusCode, Json<ProductResponse>)> {
    let id = path_extractor(id)?;
    let db = &state.conn;

    ProductService::restore(db, id).await?;

    Ok((
        StatusCode::OK,
        Json(ProductResponse {
            success: true,
            message: "Product restored successfully".to_owned(),
        }),
    ))
}
