
use serde_json::from_str;
use crate::client::Client;
use crate::errors::BoxError;

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
        Book { client: Client::new(None, None) }
    }

    pub async fn funding_currency<S>(&self, symbol: S, precision: S) -> Result<Vec<FundingCurrency>, BoxError>
        where S: Into<String>
    {
        let endpoint: String = format!("book/f{}/{}", symbol.into(), precision.into());
        let data = self.client.get(endpoint, String::new()).await?;

        let book: Vec<FundingCurrency> = from_str(data.as_str())?;

        Ok(book)
    }

    pub async fn trading_pair<S>(&self, symbol: S, precision: S) -> Result<Vec<TradingPair>, BoxError>
        where S: Into<String>
    {    
        let endpoint: String = format!("book/t{}/{}", symbol.into(), precision.into());
        let data = self.client.get(endpoint, String::new()).await?;

        let book: Vec<TradingPair> = from_str(data.as_str())?;

        Ok(book)
    }
}