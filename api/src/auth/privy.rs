use privy_rs::{
    AuthorizationContext, PrivateKey, PrivyClient,
    generated::{ResponseValue, types::WalletRpcResponse},
};
use std::env;
pub struct PClient {
    pub client: PrivyClient,
}

impl PClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let app_id = env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
        let app_secret =
            env::var("PRIVY_APP_SECRET").expect("PRIVY_APP_SECRET environment variable not set");

        let client = PrivyClient::new(app_id, app_secret)?;
        Ok(Self { client })
    }
    pub async fn sign_message(
        &self,
        wallet_address: &str,
        message: &str,
    ) -> Result<ResponseValue<WalletRpcResponse>, Box<dyn std::error::Error>> {
        let auth_key = env::var("PRIVY_SIGNER_PRIVATE_KEY")
            .expect("PRIVY_AUTH_KEY environment variable not set");
        let ctx = AuthorizationContext::new().push(PrivateKey(auth_key.to_string()));

        let res = self
            .client
            .wallets()
            .solana()
            .sign_message(wallet_address, message, &ctx, None)
            .await?;
        Ok(res)
    }
    // pub async fn getUserDetails(&self, token: String) -> Result<ResponseValue<>, Box<dyn std::error::Error>> {
    //     let res = self.client.users().get({id_token: &token}).await?;
    //     dbg!("User details: {:?}", res);
    //     Ok(res)
    // }
}
