use axum::{
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};

use crate::utils::jwt::verify_token_middleware;

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: i32,
}
pub async fn user_auth_required<B>(
    token: Option<TypedHeader<Authorization<Bearer>>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, &'static str)> {
    let token = if let Some(t) = token {
        t
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Token not provided"));
    };

    let user_claims = verify_token_middleware(token.token())?;

    let current_user = CurrentUser {
        id: user_claims.user_id,
    };

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}
