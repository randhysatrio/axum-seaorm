use axum::extract::{rejection::QueryRejection, Query};

use crate::errors::{APIResult, AppError};

pub fn query_extractor<T>(query: Result<Query<T>, QueryRejection>) -> APIResult<T> {
    match query {
        Ok(q) => Ok(q.0),
        Err(QueryRejection::FailedToDeserializeQueryString(e)) => {
            Err(AppError::InvalidQuery(e.to_string()))
        }
        _ => Err(AppError::ServerError),
    }
}
