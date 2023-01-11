use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        Path, Query,
    },
    Json,
};

use crate::errors::{APIResult, AppError};

pub type ReqBody<T> = Result<Json<T>, JsonRejection>;
pub fn body_extractor<T>(body: ReqBody<T>) -> APIResult<T> {
    match body {
        Ok(b) => Ok(b.0),
        Err(JsonRejection::JsonDataError(e)) => Err(AppError::InvalidBodyType(e.to_string())),
        Err(JsonRejection::JsonSyntaxError(e)) => Err(AppError::InvalidBodySyntax(e.to_string())),
        Err(JsonRejection::MissingJsonContentType(e)) => {
            Err(AppError::MissingBodyContentType(e.to_string()))
        }
        Err(JsonRejection::BytesRejection(e)) => Err(AppError::BodyBytesRejection(e.to_string())),
        _ => Err(AppError::ServerError),
    }
}

pub type ReqQuery<T> = Result<Query<T>, QueryRejection>;
pub fn query_extractor<T>(query: ReqQuery<T>) -> APIResult<T> {
    match query {
        Ok(q) => Ok(q.0),
        Err(QueryRejection::FailedToDeserializeQueryString(e)) => {
            Err(AppError::InvalidQuery(e.to_string()))
        }
        _ => Err(AppError::ServerError),
    }
}

pub type ReqPath<T> = Result<Path<T>, PathRejection>;
pub fn path_extractor<T>(path: ReqPath<T>) -> APIResult<T> {
    match path {
        Ok(p) => Ok(p.0),
        Err(PathRejection::FailedToDeserializePathParams(_)) => Err(AppError::InvalidPath),
        Err(PathRejection::MissingPathParams(e)) => Err(AppError::PathRequired(e.to_string())),
        _ => Err(AppError::ServerError),
    }
}
