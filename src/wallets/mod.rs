use log::debug;
use reqwest::Client;

use crate::{
    auth::Authentication,
    models::{
        request::Credentials,
        response::{auth::ServiceToken, wallets::WalletProfile, ServiceError},
    },
    util::{generate_headers, parse_response},
};

#[derive(Clone)]
pub struct WalletServiceClient {
    pub credentials: Credentials,
    pub token: Option<ServiceToken>,
    pub host: String,
    pub client: reqwest::Client,
    pub max_retries: i8,
    pub client_identifier: Option<String>,
    pub wallet_host: String,
}

impl Authentication for WalletServiceClient {
    fn token(&self) -> Option<ServiceToken> {
        self.token.clone()
    }

    fn client(&self) -> Client {
        self.client.clone()
    }

    fn credentials(&self) -> Credentials {
        self.credentials.clone()
    }

    fn set_token(&mut self, token: Option<ServiceToken>) {
        self.token = token;
    }

    fn max_retries(&self) -> i8 {
        self.max_retries
    }
}
impl WalletServiceClient {
    pub async fn get_customer(
        &mut self,
        account_id: &str,
        memo: Option<&str>,
    ) -> Result<WalletProfile, ServiceError> {
        self.attempt_token_acquisition().await;
        let wallet_url = match memo {
            Some(m) => format!("{}/user/wallet/{}?memo={}", self.wallet_host, account_id, m),
            None => format!("{}/user/wallet/{}", self.wallet_host, account_id),
        };

        debug!(
            "Getting customer for account [{}] at {}...",
            account_id, self.wallet_host
        );

        let wallet_response = self
            .client
            .get(wallet_url)
            .headers(generate_headers(
                self.token(),
                self.client_identifier.clone(),
            ))
            .send()
            .await;

        parse_response::<WalletProfile>(wallet_response).await
    }
}
