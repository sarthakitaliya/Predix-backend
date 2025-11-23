use axum::{http::StatusCode, extract::Request, middleware::Next, response::Response};


use crate::models::auth::AuthUser;

pub async fn require_admin(req: Request, next: Next) -> Result<Response, StatusCode> {
    dbg!("Require admin middleware triggered");
    let auth_user = req
        .extensions()
        .get::<AuthUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}