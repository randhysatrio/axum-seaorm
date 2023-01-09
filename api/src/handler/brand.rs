use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use ::entity::brand;

use crate::extractor::query_extractor;
use crate::services::BrandService;
use crate::{errors::APIResponse, AppState};

#[derive(Serialize, Debug)]
pub struct BrandResponse {
    success: bool,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateBrandRequest {
    name: String,
}
pub async fn create_brand(
    State(state): State<AppState>,
    Json(body): Json<CreateBrandRequest>,
) -> APIResponse<(StatusCode, Json<BrandResponse>)> {
    let db = &state.conn;

    let CreateBrandRequest { name } = body;

    let created_brand = BrandService::create(db, name).await?;

    Ok((
        StatusCode::CREATED,
        Json(BrandResponse {
            success: true,
            message: format!("Succesfully created brand with id: {}", created_brand.id),
        }),
    ))
}

#[derive(Deserialize, Debug)]
pub struct FindBrandsParams {
    keyword: Option<String>,
    page: Option<i32>,
    size: Option<i32>,
    all: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct FindBrandsResponse {
    success: bool,
    total_items: u64,
    total_page: u64,
    data: Vec<brand::Model>,
}
pub async fn find_brands(
    State(state): State<AppState>,
    params: Result<Query<FindBrandsParams>, QueryRejection>,
) -> APIResponse<(StatusCode, Json<FindBrandsResponse>)> {
    let db = &state.conn;

    let params = query_extractor(params)?;

    let FindBrandsParams {
        keyword,
        page,
        size,
        all,
    } = params;

    let (data, total_items, total_page) = BrandService::get(db, keyword, page, size, all).await?;

    Ok((
        StatusCode::OK,
        Json(FindBrandsResponse {
            success: true,
            total_items,
            total_page,
            data,
        }),
    ))
}

pub async fn delete_brand(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> APIResponse<(StatusCode, Json<BrandResponse>)> {
    let db = &state.conn;

    BrandService::delete(db, id).await?;

    Ok((
        StatusCode::OK,
        Json(BrandResponse {
            success: true,
            message: "Brands deleted successfully".to_string(),
        }),
    ))
}
