use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawClaims {
    pub sub: String, 
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub linked_accounts: Option<String>,
    pub custom_metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivyClaims {
    pub sub: String, 
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub linked_accounts: Vec<LinkedAccount>,
    pub custom_metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedAccount {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub account_type: String,
    pub address: Option<String>,
    pub chain_type: Option<String>,
    pub wallet_client_type: Option<String>,
    pub subject: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub lv: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub wallet_id: String,
    pub email: String,
    pub name: String,
    pub solana_address: String,
    pub is_admin: bool,
}
