use serde_json::from_str;
use crate::client::Client;
use crate::errors::BoxError;
use crate::endpoints::PublicEndpoint;

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingPairTicker {
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub frr: f64,
    pub bid: f64,
    pub bid_period: i64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_period: i64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64
}

#[derive(Clone)]
pub struct Ticker {
    client: Client,
}

impl Ticker {
    pub fn new() -> Self {
        Ticker {
            client: Client::new(None, None),
        }
    }

    pub async fn funding_currency<S>(&self, symbol: S) -> Result<FundingCurrency, BoxError>
        where S: Into<String>
    {
        let endpoint = PublicEndpoint::Ticker {symbol: format!("f{}", symbol.into())};
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn trading_pair<S>(&self, symbol: S) -> Result<TradingPairTicker, BoxError>
        where S: Into<String>
    {
        let endpoint = PublicEndpoint::Ticker {symbol: format!("t{}", symbol.into())};
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }
}