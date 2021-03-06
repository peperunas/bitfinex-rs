use serde_json::from_str;

use crate::client::Client;
use crate::endpoints::AuthenticatedEndpoint;
use crate::errors::BoxError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: i64,
    pub currency: String,
    _field3: Option<()>,
    pub timestamp_milli: i64,
    _field5: Option<()>,
    pub amount: f64,
    pub balance: f64,
    _field8: Option<()>,
    pub description: String,
}

#[derive(Clone)]
pub struct Ledger {
    client: Client,
}

#[derive(Serialize)]
struct HistoryParams {
    pub start: String,
    pub end: String,
    pub limit: i32,
}

impl Ledger {
    pub fn new(client: Client) -> Self {
        Ledger { client }
    }

    pub async fn get_history<S>(
        &self,
        symbol: S,
        start: u128,
        end: u128,
        limit: i32,
    ) -> Result<Vec<Entry>, BoxError>
    where
        S: Into<String>,
    {
        let endpoint = AuthenticatedEndpoint::Ledgers {
            symbol: symbol.into(),
        };
        let params = HistoryParams {
            start: format!("{}", start),
            end: format!("{}", end),
            limit,
        };
        let data = self
            .client
            .post_signed_params(&endpoint, "{}".into(), &params)
            .await?;

        Ok(from_str(data.as_str())?)
    }
}
