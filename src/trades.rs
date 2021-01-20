use serde_json::from_str;

use crate::client::Client;
use crate::endpoints::{PublicEndpoint, AuthenticatedEndpoint};
use crate::errors::BoxError;

#[derive(Serialize, Deserialize)]
pub struct Trade {
    pub id: i64,
    pub pair: String,
    pub execution_timestap: i64,
    pub order_id: i32,
    pub execution_amount: f64,
    pub execution_price: f64,
    pub order_type: String,
    pub order_price: f64,
    pub maker: i32,
    pub fee: f64,
    pub fee_currency: String,
}

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
    pub fn new() -> Self {
        Trades {
            client: Client::new(None, None),
        }
    }

    pub async fn funding_currency<S>(&self, symbol: S) -> Result<Vec<FundingCurrency>, BoxError>
        where S: Into<String>
    {
        let endpoint = PublicEndpoint::Trades { symbol: format!("f{}", symbol.into()) };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn trading_pair<S>(&self, symbol: S) -> Result<Vec<TradingPair>, BoxError>
        where S: Into<String>
    {
        let endpoint = PublicEndpoint::Trades { symbol: format!("t{}", symbol.into()) };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn history<S: ToString>(&self, symbol: S) -> Result<Vec<Trade>, BoxError>
    {
        let endpoint = AuthenticatedEndpoint::OrdersHistory {symbol: Some(symbol.to_string())};
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }

    pub async fn generated_by_order<S: ToString>(&self, symbol: S, order_id: u64) -> Result<Vec<Trade>, BoxError>
    {
        let endpoint = AuthenticatedEndpoint::OrderTrades {symbol: symbol.to_string(), order_id};
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }
}