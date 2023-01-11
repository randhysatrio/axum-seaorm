use axum::{
    extract::{rejection::PathRejection, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use ::entity::category;

use crate::services::CategoryService;
use crate::AppState;
use crate::{errors::APIResponse, extractor::path_extractor};

#[derive(Deserialize, Debug)]
pub struct CategoryQuery {
    keyword: Option<String>,
    all: Option<bool>,
    page: Option<i32>,
    size: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct CategoryResponse {
    success: bool,
    total_items: u64,
    total_page: u64,
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

    let (data, total_items, total_page) =
        CategoryService::get(db, keyword, all, page, size).await?;

    Ok((
        StatusCode::OK,
        Json(CategoryResponse {
            success: true,
            total_items,
            total_page,
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
    id: Result<Path<i32>, PathRejection>,
) -> APIResponse<(StatusCode, Json<CategoryCRUDResponse>)> {
    let id = path_extractor(id)?;
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

pub async fn restore_category(
    State(state): State<AppState>,
    id: Result<Path<i32>, PathRejection>,
) -> APIResponse<(StatusCode, Json<CategoryCRUDResponse>)> {
    let id = path_extractor(id)?;
    let db = &state.conn;

    CategoryService::restore(db, id).await?;

    Ok((
        StatusCode::OK,
        Json(CategoryCRUDResponse {
            success: true,
            message: "Category restored successfully!",
        }),
    ))
}
