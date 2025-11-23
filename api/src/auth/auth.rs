use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use std::env;

use crate::models::auth::{AuthUser, PrivyClaims, RawClaims};


pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    dbg!("Auth middleware triggered");
    let auth_header = headers.get("privy-id-token").and_then(|v| v.to_str().ok());
    let access_token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let token = match auth_header {
        Some(c) => c.to_string(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let access_token = match access_token {
        Some(c) => c.to_string(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let public_key = env::var("PRIVY_VERIFICATION_PEM")
        .expect("PRIVY_VERIFICATION_PEM environment variable not set");
    let app_id = env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_issuer(&["privy.io"]);
    validation.set_audience(&[&app_id]);

    let decoding_key = DecodingKey::from_ec_pem(public_key.as_bytes()).expect("Invalid public key");

    let token_data =
        decode::<RawClaims>(&token, &decoding_key, &validation).expect("Token verification failed");
    let data = PrivyClaims {
        sub: token_data.claims.sub,
        iss: token_data.claims.iss,
        aud: token_data.claims.aud,
        exp: token_data.claims.exp,
        iat: token_data.claims.iat,
        linked_accounts: match token_data.claims.linked_accounts {
            Some(accounts_str) => serde_json::from_str(&accounts_str).unwrap_or_default(),
            None => return Err(StatusCode::UNAUTHORIZED),
        },
        custom_metadata: token_data.claims.custom_metadata,
    };
    // return error if no linked accounts
    if data.linked_accounts.is_empty() {
        dbg!("No linked accounts found");
        return Err(StatusCode::UNAUTHORIZED);
    }
    let email = data
        .linked_accounts
        .iter()
        .find(|acc| acc.account_type == "google_oauth")
        .and_then(|acc| acc.email.clone());
    let name = data
        .linked_accounts
        .iter()
        .find(|acc| acc.account_type == "google_oauth")
        .and_then(|acc| acc.name.clone());
    let solana_address = data
        .linked_accounts
        .iter()
        .find(|acc| acc.account_type == "wallet" && acc.chain_type.as_deref() == Some("solana"))
        .and_then(|acc| acc.address.clone());
    let wallet_id = data
        .linked_accounts
        .iter()
        .find(|acc| acc.account_type == "wallet" && acc.chain_type.as_deref() == Some("solana"))
        .and_then(|acc| acc.id.clone());
    // Check if required fields are available before constructing AuthUser
    let email = match email {
        Some(e) => e,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let is_admin = email == env::var("ADMIN_EMAIL").unwrap_or_default();

    let solana_address = match solana_address {
        Some(addr) => addr,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let wallet_id = match wallet_id {
        Some(id) => id,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let name = name.unwrap_or_default();

    let auth_user = AuthUser {
        access_token,
        wallet_id,
        email,
        name,
        solana_address,
        is_admin,
    };
    dbg!("Auth user constructed:", &auth_user);
    req.extensions_mut().insert(auth_user);
    Ok(next.run(req).await)
}
