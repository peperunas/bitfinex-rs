use serde_json::from_str;
use crate::client::Client;
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
    pub fee_currency: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingPair {
    pub mts: i64,
    pub amount: f64,
    pub price: f64,
    pub rate: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub mts: i64,
    pub amount: f64,
    pub price: f64,
    pub rate: f64,
    pub period: i64
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
        let endpoint: String = format!("trades/f{}/hist", symbol.into());
        let data = self.client.get(endpoint, String::new()).await?;

        let trades: Vec<FundingCurrency> = from_str(data.as_str())?;

        Ok(trades)
    }

    pub async fn trading_pair<S>(&self, symbol: S) -> Result<Vec<TradingPair>, BoxError>
        where S: Into<String>
    {
        let endpoint: String = format!("trades/t{}/hist", symbol.into());
        let data = self.client.get(endpoint, String::new()).await?;

        let trades: Vec<TradingPair> = from_str(data.as_str())?;

        Ok(trades)
    }

    pub async fn history<S>(&self, symbol: S) -> Result<Vec<Trade>, BoxError>
        where S: Into<String>
    {
        let payload: String = format!("{}", "{}");
        let request: String = format!("trades/t{}/hist", symbol.into());

        self.trades(request, payload).await
    }

    pub async fn generated_by_order<S>(&self, symbol: S, order_id: S) -> Result<Vec<Trade>, BoxError>
        where S: Into<String>
    {
        let payload: String = format!("{}", "{}");
        let request: String = format!("order/t{}:{}/trades", symbol.into(), order_id.into());

        self.trades(request, payload).await
    }   

    pub async fn trades<S>(&self, request: S, payload: S) -> Result<Vec<Trade>, BoxError>
        where S: Into<String>
    {
        let data = self.client.post_signed(request.into(), payload.into()).await?;

        let orders: Vec<Trade> = from_str(data.as_str())?;

        Ok(orders)
    }    
}