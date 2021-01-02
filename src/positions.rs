use serde::{Deserialize, Deserializer};
use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::client::Client;
use crate::errors::BoxError;

fn coercible<'de, D, T>(deserializer: D) -> Result<T, D::Error> where
    T: DeserializeOwned,
    D: Deserializer<'de> {
    use serde::de::Error;

    let coercible_string = String::deserialize(deserializer)?;

    from_str(&coercible_string).map_err(Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionMeta {
    reason: String,
    order_id: u64,
    order_id_oppo: u64,
    liq_stage: Option<String>,
    #[serde(deserialize_with = "coercible")]
    trade_price: f64,
    #[serde(deserialize_with = "coercible")]
    trade_amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    symbol: String,
    status: String,
    amount: f64,
    base_price: f64,
    margin_funding: f64,
    margin_funding_type: u8,
    pl: f64,
    pl_perc: f64,
    price_liq: f64,
    leverage: f64,
    #[serde(skip_serializing)]
    _placeholder1: Option<String>,
    position_id: u64,
    mts_create: Option<u64>,
    mts_update: Option<u64>,
    #[serde(skip_serializing)]
    _placeholder2: Option<String>,
    position_type: u64,
    #[serde(skip_serializing)]
    _placeholder3: Option<String>,
    collateral: f64,
    collateral_min: f64,
    meta: Option<PositionMeta>,
}

#[derive(Clone)]
pub struct Positions {
    client: Client,
}

impl Positions {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Positions {
            client: Client::new(api_key, secret_key),
        }
    }

    pub async fn active_positions(&self) -> Result<Vec<Position>, BoxError> {
        let post = self.client.post_signed("positions".into(), "{}".into()).await?;

        Ok(from_str(&post)?)
    }
}