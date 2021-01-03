use serde::{Deserialize, Deserializer};
use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::client::Client;
use crate::errors::BoxError;
use std::collections::HashMap;

fn coercible<'de, D, T>(deserializer: D) -> Result<T, D::Error> where
    T: DeserializeOwned,
    D: Deserializer<'de> {
    use serde::de::Error;

    let coercible_string = String::deserialize(deserializer)?;

    from_str(&coercible_string).map_err(Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    meta: Option<HashMap<String, serde_json::Value>>,
}

impl Position {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn base_price(&self) -> f64 {
        self.base_price
    }
    pub fn margin_funding(&self) -> f64 {
        self.margin_funding
    }
    pub fn margin_funding_type(&self) -> u8 {
        self.margin_funding_type
    }
    pub fn pl(&self) -> f64 {
        self.pl
    }
    pub fn pl_perc(&self) -> f64 {
        self.pl_perc
    }
    pub fn price_liq(&self) -> f64 {
        self.price_liq
    }
    pub fn leverage(&self) -> f64 {
        self.leverage
    }
    pub fn position_id(&self) -> u64 {
        self.position_id
    }
    pub fn mts_create(&self) -> Option<u64> {
        self.mts_create
    }
    pub fn mts_update(&self) -> Option<u64> {
        self.mts_update
    }
    pub fn position_type(&self) -> u64 {
        self.position_type
    }
    pub fn collateral(&self) -> f64 {
        self.collateral
    }
    pub fn collateral_min(&self) -> f64 {
        self.collateral_min
    }
    pub fn meta(&self) -> &Option<HashMap<String, serde_json::Value>> {
        &self.meta
    }
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