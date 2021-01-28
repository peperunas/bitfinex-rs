use serde_json::from_str;

use crate::client::Client;
use crate::endpoints::{AuthenticatedEndpoint, PublicEndpoint};
use crate::errors::BoxError;
use crate::responses::TradeResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingPair {
    pub mts: i64,
    pub amount: f64,
    pub price: f64,
    pub rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub mts: i64,
    pub amount: f64,
    pub price: f64,
    pub rate: f64,
    pub period: i64,
}

#[derive(Clone)]
pub struct Trades {
    client: Client,
}

impl Trades {
    pub fn new(client: Client) -> Self {
        Trades { client }
    }

    pub async fn funding_currency<S>(&self, symbol: S) -> Result<Vec<FundingCurrency>, BoxError>
    where
        S: Into<String>,
    {
        let endpoint = PublicEndpoint::Trades {
            symbol: format!("f{}", symbol.into()),
        };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn trading_pair<S>(&self, symbol: S) -> Result<Vec<TradingPair>, BoxError>
    where
        S: Into<String>,
    {
        let endpoint = PublicEndpoint::Trades {
            symbol: format!("t{}", symbol.into()),
        };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn history<S: ToString>(&self, symbol: S) -> Result<Vec<TradeResponse>, BoxError> {
        let endpoint = AuthenticatedEndpoint::OrdersHistory {
            symbol: Some(symbol.to_string()),
        };
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }

    pub async fn generated_by_order<S: ToString>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<Vec<TradeResponse>, BoxError> {
        let endpoint = AuthenticatedEndpoint::OrderTrades {
            symbol: symbol.to_string(),
            order_id,
        };
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }
}
