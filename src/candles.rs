use serde_json::from_str;

use crate::client::Client;
use crate::endpoints::PublicEndpoint;
use crate::errors::BoxError;

#[derive(Serialize, Deserialize)]
pub enum CandlesTimeFrame {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "3h")]
    ThreeHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1D")]
    OneDay,
    #[serde(rename = "7D")]
    OneWeek,
    #[serde(rename = "14D")]
    TwoWeeks,
}

impl ToString for CandlesTimeFrame {
    fn to_string(&self) -> String {
        match self {
            CandlesTimeFrame::OneMinute => { "1m".into() }
            CandlesTimeFrame::FiveMinutes => { "5m".into() }
            CandlesTimeFrame::FifteenMinutes => { "15m".into() }
            CandlesTimeFrame::ThirtyMinutes => { "30m".into() }
            CandlesTimeFrame::OneHour => { "1h".into() }
            CandlesTimeFrame::ThreeHours => { "3h".into() }
            CandlesTimeFrame::SixHours => { "6h".into() }
            CandlesTimeFrame::TwelveHours => { "12h".into() }
            CandlesTimeFrame::OneDay => { "1D".into() }
            CandlesTimeFrame::OneWeek => { "7D".into() }
            CandlesTimeFrame::TwoWeeks => { "14D".into() }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum CandlesSection {
    #[serde(rename = "last")]
    Last,
    #[serde(rename = "hist")]
    Hist,
}

impl ToString for CandlesSection {
    fn to_string(&self) -> String {
        match self {
            CandlesSection::Last => { "last".into() }
            CandlesSection::Hist => { "hist".into() }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CandleHistoryParams {
    /// Number of candles requested (Max: 10000)
    pub limit: Option<i32>,

    /// Filter start (ms)
    pub start: Option<i64>,

    /// Filter end (ms)
    pub end: Option<i64>,

    /// Sorts the results from old > new
    pub sort: Option<bool>,
}

impl CandleHistoryParams {
    pub fn new() -> Self {
        Self {
            limit: Some(120),
            sort: Some(false),
            start: None,
            end: None,
        }
    }

    pub fn to_query(&self) -> String {
        format!("{}={}&{}={}&{}={}&{}={}",
                "limit", self.limit
                    .map(|a| a.to_string())
                    .unwrap_or("".into()),
                "start", self.start
                    .map(|a| a.to_string())
                    .unwrap_or("".into()),
                "end", self.end
                    .map(|a| a.to_string())
                    .unwrap_or("".into()),
                "sort", self.sort
                    .map(|a| if a { "1" } else { "0" })
                    .unwrap_or("".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

#[derive(Clone)]
pub struct Candles {
    client: Client,
}

// TODO: funding missing
impl Candles {
    pub fn new() -> Self {
        Candles {
            client: Client::new(None, None),
        }
    }

    pub async fn last<S>(&self, symbol: S, timeframe: CandlesTimeFrame) -> Result<Candle, BoxError>
        where S: Into<String>
    {
        let endpoint = PublicEndpoint::Candles { symbol: symbol.into(), timeframe, section: CandlesSection::Last, funding_period: None };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }

    pub async fn history<S>(
        &self,
        symbol: S,
        timeframe: CandlesTimeFrame,
    ) -> Result<Vec<Candle>, BoxError>
        where S: Into<String>
    {
        let endpoint = PublicEndpoint::Candles { symbol: symbol.into(), timeframe, section: CandlesSection::Hist, funding_period: None };
        let data = self.client.get(endpoint).await?;

        Ok(from_str(data.as_str())?)
    }
}