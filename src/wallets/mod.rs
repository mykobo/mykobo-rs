use std::env;

use log::debug;
use reqwest::Client;
use serde_json::json;

use crate::{
    models::{
        request::wallets::RegisterWalletRequest,
        response::{
            auth::ServiceToken,
            wallets::{UserWallet, WalletProfile},
            ServiceError,
        },
    },
    util::{generate_headers, parse_response},
};

#[derive(Clone)]
pub struct WalletServiceClient {
    pub host: String,
    pub client: Client,
    pub max_retries: i8,
    pub client_identifier: Option<String>,
    pub wallet_host: String,
}

impl WalletServiceClient {
    pub fn new(max_retries: i8) -> Self {
        let wallet_host = env::var("WALLET_HOST").expect("WALLET_HOST must be set");

        Self {
            host: wallet_host.clone(),
            client: Client::new(),
            max_retries,
            client_identifier: Some("mykobo-rs".to_string()),
            wallet_host,
        }
    }

    pub async fn get_wallet_profile(
        &self,
        token: Option<ServiceToken>,
        account_id: &str,
        memo: Option<&str>,
    ) -> Result<WalletProfile, ServiceError> {
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
            .headers(generate_headers(token, self.client_identifier.clone()))
            .send()
            .await;

        parse_response::<WalletProfile>(wallet_response).await
    }

    pub async fn register_wallet(
        &self,
        token: Option<ServiceToken>,
        request: RegisterWalletRequest,
    ) -> Result<UserWallet, ServiceError> {
        debug!("Registering wallet for user...");
        let payload = json!(request).to_string();
        let wallet_response = self
            .client
            .post(format!("{}/wallet/register", self.wallet_host))
            .headers(generate_headers(token, self.client_identifier.clone()))
            .body(payload)
            .send()
            .await;

        parse_response::<UserWallet>(wallet_response).await
    }
}
