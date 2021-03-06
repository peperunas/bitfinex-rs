use serde_json::from_str;

use crate::client::Client;
use crate::endpoints::{AuthenticatedEndpoint, MarginInfoKey};
use crate::errors::BoxError;
use crate::responses::{AccountFees, WalletTransferResponse};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub wallet_type: WalletKind,
    pub currency: String,
    pub balance: f64,
    pub unsettled_interest: f64,
    pub balance_available: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WalletKind {
    #[serde(rename = "exchange")]
    Exchange,
    #[serde(rename = "margin")]
    Margin,
    #[serde(rename = "funding")]
    Funding,
}

#[derive(Serialize, Deserialize)]
pub struct MarginBase {
    key: String,
    pub margin: Base,
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    pub user_profit_loss: f64,
    pub user_swaps: f64,
    pub margin_balance: f64,
    pub margin_net: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MarginSymbol {
    key: String,
    symbol: String,
    pub margin: Symbol,
}

#[derive(Serialize, Deserialize)]
pub struct Symbol {
    pub tradable_balance: f64,
    pub gross_balance: f64,
    pub buy: f64,
    pub sell: f64,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FundingInfo {
    key: String,
    symbol: String,
    pub funding: Funding,
}

#[derive(Serialize, Deserialize)]
pub struct Funding {
    pub yield_loan: f64,
    pub yield_lend: f64,
    pub duration_loan: f64,
    pub duration_lend: f64,
}

#[derive(Clone)]
pub struct Account {
    client: Client,
}

impl Account {
    pub fn new(client: Client) -> Self {
        Account { client }
    }

    pub async fn get_wallets(&self) -> Result<Vec<Wallet>, BoxError> {
        let endpoint = AuthenticatedEndpoint::Wallets;
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn transfer_between_wallets(
        &self,
        from: &WalletKind,
        to: &WalletKind,
        currency: String,
        amount: f64,
    ) -> Result<WalletTransferResponse, BoxError> {
        let endpoint = AuthenticatedEndpoint::WalletTransfer;
        let payload =
            json!({"from": from, "to": to, "currency": currency, "amount": amount.to_string()});

        let response = self
            .client
            .post_signed(&endpoint, payload.to_string())
            .await?;

        Ok(from_str(&response)?)
    }

    pub async fn margin_base(&self) -> Result<MarginBase, BoxError> {
        let endpoint = AuthenticatedEndpoint::MarginInfo {
            key: MarginInfoKey::Base,
        };
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }

    pub async fn margin_symbol<S: ToString>(&self, key: S) -> Result<MarginSymbol, BoxError> {
        let endpoint = AuthenticatedEndpoint::MarginInfo {
            key: MarginInfoKey::Symbol(key.to_string()),
        };
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn funding_info<S>(&self, key: S) -> Result<FundingInfo, BoxError>
    where
        S: Into<String>,
    {
        let endpoint = AuthenticatedEndpoint::FundingInfo { symbol: key.into() };
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn account_summary(&self) -> Result<AccountFees, BoxError> {
        let endpoint = AuthenticatedEndpoint::Summary;
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }
}
