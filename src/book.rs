use serde_json::from_str;

use crate::client::Client;
use crate::endpoints::PublicEndpoint;
use crate::errors::BoxError;

#[derive(Serialize, Deserialize)]
pub enum BookPrecision {
    #[serde(rename = "P0")]
    P0,
    #[serde(rename = "P1")]
    P1,
    #[serde(rename = "P2")]
    P2,
    #[serde(rename = "P3")]
    P3,
    #[serde(rename = "P4")]
    P4,
    #[serde(rename = "R0")]
    R0,
}

impl ToString for BookPrecision {
    fn to_string(&self) -> String {
        match self {
            BookPrecision::P0 => "P0".into(),
            BookPrecision::P1 => "P1".into(),
            BookPrecision::P2 => "P2".into(),
            BookPrecision::P3 => "P3".into(),
            BookPrecision::P4 => "P4".into(),
            BookPrecision::R0 => "R0".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingPair {
    pub price: f64,
    pub count: i64,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub rate: f64,
    pub period: f64,
    pub count: i64,
    pub amount: f64,
}

#[derive(Clone)]
pub struct Book {
    client: Client,
}

// Trading: if AMOUNT > 0 then bid else ask; Funding: if AMOUNT < 0 then bid else ask;
#[derive(Serialize, Deserialize, Debug)]
pub struct RawBook {
    pub order_id: i64,
    pub price: f64,
    pub amount: f64,
}

impl Book {
    pub fn new() -> Self {
        Book {
            client: Client::new(None, None),
        }
    }

    pub async fn funding_currency<S>(
        &self,
        symbol: S,
        precision: BookPrecision,
    ) -> Result<Vec<FundingCurrency>, BoxError>
    where
        S: Into<String>,
    {
        let endpoint = PublicEndpoint::Book {
            symbol: format!("f{}", symbol.into()),
            precision,
        };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn trading_pair<S>(
        &self,
        symbol: S,
        precision: BookPrecision,
    ) -> Result<Vec<TradingPair>, BoxError>
    where
        S: Into<String>,
    {
        let endpoint = PublicEndpoint::Book {
            symbol: format!("t{}", symbol.into()),
            precision,
        };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }
}
