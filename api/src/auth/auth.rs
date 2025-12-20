use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use std::env;

use crate::models::auth::{AuthUser, Jwks, PrivyClaims, RawClaims};

pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    dbg!("Auth middleware triggered");
    let auth_header = headers.get("privy-id-token").and_then(|v| v.to_str().ok());
    let token = match auth_header {
        Some(c) => c.to_string(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    dbg!("Token extracted:", &token);
    let header = decode_header(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    dbg!("Decoded header:", &header);
    let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;
    let app_id = env::var("PRIVY_APP_ID").unwrap();
    let jwks_url = format!("https://auth.privy.io/api/v1/apps/{}/jwks.json", app_id);
    let jwks: Jwks = reqwest::get(jwks_url)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .json()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let jwk = jwks
        .keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    dbg!("JWK found:", &jwk);
    let app_id = env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_issuer(&["privy.io"]);
    validation.set_audience(&[&app_id]);
    dbg!("Validation params:", &validation);
    let decoding_key =
        DecodingKey::from_ec_components(&jwk.x, &jwk.y).map_err(|_| StatusCode::UNAUTHORIZED)?;
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
    dbg!("Privy claims:", &data);
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
    // Return empty response if any are missing

    let is_admin = email
        .as_deref()
        .is_some_and(|e| e == env::var("ADMIN_EMAIL").unwrap_or_default());
    let solana_address = match solana_address {
        Some(addr) => addr,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let auth_user = AuthUser {
        wallet_id,
        email,
        name,
        solana_address,
        is_admin,
    };
    dbg!("Authenticated user:", &auth_user);
    req.extensions_mut().insert(auth_user);
    Ok(next.run(req).await)
}
