use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use ::entity::category;

use crate::errors::{APIResponse, AppError};
use crate::services::CategoryService;
use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct CategoryQuery {
    keyword: Option<String>,
    all: Option<bool>,
    page: Option<u64>,
    size: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct CategoryResponse {
    success: bool,
    count: u64,
    data: Vec<category::Model>,
}
pub async fn find_category(
    State(state): State<AppState>,
    Query(query): Query<CategoryQuery>,
) -> APIResponse<(StatusCode, Json<CategoryResponse>)> {
    let db = &state.conn;

    let CategoryQuery {
        keyword,
        all,
        size,
        page,
    } = query;

    let (count, data) = CategoryService::get(db, keyword, all, page, size).await?;

    Ok((
        StatusCode::OK,
        Json(CategoryResponse {
            success: true,
            count,
            data,
        }),
    ))
}

#[derive(Serialize, Debug)]
pub struct CategoryCRUDResponse {
    success: bool,
    message: &'static str,
}

#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
    name: String,
}
pub async fn create_category(
    State(state): State<AppState>,
    Json(body): Json<CreateCategoryRequest>,
) -> APIResponse<(StatusCode, Json<CategoryCRUDResponse>)> {
    let db = &state.conn;

    let CreateCategoryRequest { name } = body;

    CategoryService::create(db, name).await?;

    Ok((
        StatusCode::CREATED,
        Json(CategoryCRUDResponse {
            success: true,
            message: "Category created successfully!",
        }),
    ))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> APIResponse<(StatusCode, Json<CategoryCRUDResponse>)> {
    let db = &state.conn;

    CategoryService::delete(db, id).await?;

    Ok((
        StatusCode::OK,
        Json(CategoryCRUDResponse {
            success: true,
            message: "Category deleted successfully!",
        }),
    ))
}
