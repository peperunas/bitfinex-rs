use crate::book::BookPrecision;
use crate::candles::{CandlesSection, CandlesTimeFrame};

// TODO: incomplete
#[derive(Debug)]
pub enum PublicEndpoint {
    Status,
    Tickers {
        symbols: Vec<String>,
    },
    Ticker {
        symbol: String,
    },
    TickersHistory {
        symbols: Vec<String>,
    },
    Trades {
        symbol: String,
    },
    Book {
        symbol: String,
        precision: BookPrecision,
    },
    Candles {
        timeframe: CandlesTimeFrame,
        symbol: String,
        section: CandlesSection,
        funding_period: Option<String>,
    },
}

impl PublicEndpoint {
    const HOST: &'static str = "https://api-pub.bitfinex.com";
    const PATH: &'static str = "/v2";
}

impl ToString for PublicEndpoint {
    fn to_string(&self) -> String {
        let mut endpoint = format!("{}{}", PublicEndpoint::HOST, PublicEndpoint::PATH);

        match self {
            PublicEndpoint::Status => endpoint.push_str("/platform/status"),
            PublicEndpoint::Tickers { symbols } => {
                let joined_syms = symbols.join(",");
                endpoint.push_str(&*format!("/tickers?symbols={}", joined_syms));
            }
            PublicEndpoint::Ticker { symbol } => endpoint.push_str(&format!("/ticker/{}", symbol)),
            PublicEndpoint::TickersHistory { symbols } => {
                let joined_syms = symbols.join(",");
                endpoint.push_str(&*format!("/tickers/hist?symbols={}", joined_syms));
            }
            PublicEndpoint::Trades { symbol } => {
                endpoint.push_str(&format!("/ticker/{}/hist", symbol))
            }
            PublicEndpoint::Book { symbol, precision } => {
                endpoint.push_str(&format!("/book/{}/{}", symbol, precision.to_string()))
            }
            PublicEndpoint::Candles {
                symbol,
                funding_period,
                section,
                timeframe,
            } => {
                let query = match funding_period {
                    Some(period) => {
                        format!(
                            "/candles/trade:{}:{}:{}/{}",
                            timeframe.to_string(),
                            symbol.to_string(),
                            period.to_string(),
                            section.to_string()
                        )
                    }
                    None => {
                        format!(
                            "/candles/trade:{}:{}/{}",
                            timeframe.to_string(),
                            symbol.to_string(),
                            section.to_string()
                        )
                    }
                };

                endpoint.push_str(&query);
            }
        }

        endpoint
    }
}

// TODO: incomplete
#[derive(Debug)]
pub enum AuthenticatedEndpoint {
    Wallets,
    RetrieveOrders,
    SubmitOrder,
    UpdateOrder,
    CancelOrder,
    OrdersHistory { symbol: Option<String> },
    OrderTrades { symbol: String, order_id: u64 },
    Trades { symbol: String },
    Ledgers { symbol: String },
    MarginInfo { key: MarginInfoKey },
    RetrievePositions,
    ClaimPosition,
    PositionsHistory,
    PositionsSnapshot,
    PositionsAudit,
    DerivativePositionCollateral,
    DerivativePositionCollateralLimits,
    UserInfo,
    Summary,
    FundingInfo { symbol: String },
    WalletTransfer,
}

impl AuthenticatedEndpoint {
    pub const HOST: &'static str = "https://api.bitfinex.com";
    const READ_PATH: &'static str = "/v2/auth/r";
    const WRITE_PATH: &'static str = "/v2/auth/w";
    const CALC_PATH: &'static str = "/v2/auth/calc";

    pub fn path(&self) -> String {
        // unwrapping since HOST is always present in an AuthenticatedEndpoint
        self.to_string()
            .strip_prefix(AuthenticatedEndpoint::HOST)
            .unwrap()
            .to_owned()
    }
}

impl ToString for AuthenticatedEndpoint {
    fn to_string(&self) -> String {
        let mut endpoint = String::from(AuthenticatedEndpoint::HOST);

        match self {
            AuthenticatedEndpoint::Wallets => {
                endpoint.push_str(&format!("{}/wallets", AuthenticatedEndpoint::READ_PATH))
            }
            AuthenticatedEndpoint::RetrieveOrders => {
                endpoint.push_str(&format!("{}/orders", AuthenticatedEndpoint::READ_PATH))
            }
            AuthenticatedEndpoint::SubmitOrder => endpoint.push_str(&format!(
                "{}/order/submit",
                AuthenticatedEndpoint::WRITE_PATH
            )),
            AuthenticatedEndpoint::UpdateOrder => endpoint.push_str(&format!(
                "{}/order/update",
                AuthenticatedEndpoint::WRITE_PATH
            )),
            AuthenticatedEndpoint::CancelOrder => endpoint.push_str(&format!(
                "{}/order/cancel",
                AuthenticatedEndpoint::WRITE_PATH
            )),
            AuthenticatedEndpoint::OrdersHistory { symbol } => endpoint.push_str(
                match symbol {
                    Some(symbol) => {
                        format!(
                            "{}/orders/{}/hist",
                            AuthenticatedEndpoint::READ_PATH,
                            symbol
                        )
                    }
                    None => {
                        format!("{}/orders/hist", AuthenticatedEndpoint::READ_PATH)
                    }
                }
                .as_str(),
            ),
            AuthenticatedEndpoint::OrderTrades { symbol, order_id } => endpoint.push_str(&format!(
                "{}/order/{}:{}/trades",
                AuthenticatedEndpoint::READ_PATH,
                symbol,
                order_id
            )),
            AuthenticatedEndpoint::Trades { symbol } => endpoint.push_str(&format!(
                "{}/trades/{}/hist",
                AuthenticatedEndpoint::READ_PATH,
                symbol
            )),
            AuthenticatedEndpoint::Ledgers { symbol } => endpoint.push_str(&format!(
                "{}/ledgers/{}/hist",
                AuthenticatedEndpoint::READ_PATH,
                symbol
            )),
            AuthenticatedEndpoint::MarginInfo { key } => endpoint.push_str(&format!(
                "{}/info/margin/{}",
                AuthenticatedEndpoint::READ_PATH,
                key.to_string()
            )),
            AuthenticatedEndpoint::RetrievePositions => {
                endpoint.push_str(&format!("{}/positions", AuthenticatedEndpoint::READ_PATH))
            }
            AuthenticatedEndpoint::ClaimPosition => endpoint.push_str(&format!(
                "{}/position/claim",
                AuthenticatedEndpoint::WRITE_PATH
            )),
            AuthenticatedEndpoint::PositionsHistory => endpoint.push_str(&format!(
                "{}/positions/hist",
                AuthenticatedEndpoint::READ_PATH
            )),
            AuthenticatedEndpoint::PositionsSnapshot => endpoint.push_str(&format!(
                "{}/positions/snap",
                AuthenticatedEndpoint::READ_PATH
            )),
            AuthenticatedEndpoint::PositionsAudit => endpoint.push_str(&format!(
                "{}/positions/audit",
                AuthenticatedEndpoint::READ_PATH
            )),
            AuthenticatedEndpoint::DerivativePositionCollateral => endpoint.push_str(&format!(
                "{}/deriv/collateral/set",
                AuthenticatedEndpoint::READ_PATH
            )),
            AuthenticatedEndpoint::DerivativePositionCollateralLimits => {
                endpoint.push_str(&format!(
                    "{}/deriv/collateral/limits",
                    AuthenticatedEndpoint::CALC_PATH
                ))
            }
            AuthenticatedEndpoint::UserInfo => {
                endpoint.push_str(&format!("{}/info/user", AuthenticatedEndpoint::READ_PATH))
            }
            AuthenticatedEndpoint::Summary => {
                endpoint.push_str(&format!("{}/summary", AuthenticatedEndpoint::READ_PATH))
            }
            AuthenticatedEndpoint::FundingInfo { symbol } => endpoint.push_str(&format!(
                "{}/info/funding/{}",
                AuthenticatedEndpoint::READ_PATH,
                symbol
            )),
            AuthenticatedEndpoint::WalletTransfer => {
                endpoint.push_str(&format!("{}/transfer", AuthenticatedEndpoint::WRITE_PATH))
            }
        }

        endpoint
    }
}

#[derive(Debug)]
pub enum MarginInfoKey {
    Base,
    Symbol(String),
    SymAll,
}

impl ToString for MarginInfoKey {
    fn to_string(&self) -> String {
        match self {
            MarginInfoKey::Base => "base".into(),
            MarginInfoKey::Symbol(s) => s.to_owned(),
            MarginInfoKey::SymAll => "sym_all".into(),
        }
    }
}
