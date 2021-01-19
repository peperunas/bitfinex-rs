use chrono::{DateTime, TimeZone};
use serde_json::from_str;
use std::fmt::Display;

use crate::client::Client;
use crate::errors::BoxError;
use serde::{Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone)]
pub struct ActiveOrder {
    pub id: i64,
    pub group_id: Option<i32>,
    pub client_id: i64,
    pub symbol: String,
    pub creation_timestamp: i64,
    pub update_timestamp: i64,
    pub amount: f64,
    pub amount_original: f64,
    pub order_type: String,
    pub previous_order_type: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    
    pub flags: Option<i32>,                   
    pub order_status: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: Option<f64>,
    pub price_aux_limit: Option<f64>,
    
    #[serde(skip_serializing)]
    __placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,    
    
    pub notify: i32,
    pub hidden: i32,
    pub placed_id: Option<i32>                      
}

#[derive(Clone, Deserialize)]
pub enum OrderKind {
    Limit,
    ExchangeLimit,
    Market,
    ExchangeMarket,
    Stop,
    ExchangeStop,
    StopLimit,
    ExchangeStopLimit,
    TrailingStop,
    Fok,
    ExchangeFok,
    Ioc,
    ExchangeIoc
}

impl OrderKind {
    const fn as_str(&self) -> &'static str {
        match *self {
            OrderKind::Limit => {"LIMIT"}
            OrderKind::ExchangeLimit => {"EXCHANGE LIMIT"}
            OrderKind::Market => {"MARKET"}
            OrderKind::ExchangeMarket => {"EXCHANGE MARKET"}
            OrderKind::Stop => {"STOP"}
            OrderKind::ExchangeStop => {"EXCHANGE STOP"}
            OrderKind::StopLimit => {"STOP LIMIT"}
            OrderKind::ExchangeStopLimit => {"EXCHANGE STOP LIMIT"}
            OrderKind::TrailingStop => {"TRAILING STOP"}
            OrderKind::Fok => {"FOK"}
            OrderKind::ExchangeFok => {"EXCHANGE FOK"}
            OrderKind::Ioc => {"IOC"}
            OrderKind::ExchangeIoc => {"EXCHANGE IOC"}
        }
    }
}

impl ToString for OrderKind {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Serialize for OrderKind {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderMeta {
    aff_code: String
}

impl OrderMeta {
    pub fn new(aff_code: String) -> Self {
        OrderMeta { aff_code }
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct OrderFlag: u32 {
        const NONE = 0;
        const HIDDEN = 64;
        const CLOSE = 512;
        const REDUCE_ONLY = 1024;
        const POST_ONLY = 4096;
        const OCO = 16384;
        const NO_VAR_RATES = 524288;
    }
}

impl Default for OrderFlag {
    fn default() -> Self {
        OrderFlag::NONE
    }
}

#[derive(Serialize, Clone)]
pub struct OrderForm {
    /// Group id for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<u32>,
    /// Should be unique in the day (UTC) (not enforced)
    #[serde(skip_serializing_if = "Option::is_none")]
    cid: Option<u32>,
    /// Order Type: LIMIT, EXCHANGE LIMIT, MARKET, EXCHANGE MARKET,
    /// STOP, EXCHANGE STOP, STOP LIMIT, EXCHANGE STOP LIMIT,
    /// TRAILING STOP, EXCHANGE TRAILING STOP, FOK,
    /// EXCHANGE FOK, IOC, EXCHANGE IOC
    #[serde(rename="type")]
    order_type: OrderKind,
    /// Symbol for desired pair
    symbol: String,
    /// Price of order
    price: String,
    /// Amount of order (positive for buy, negative for sell)
    amount: String,
    /// Optional see https://docs.bitfinex.com/v2/docs/flag-values
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<u32>,
    /// Set the leverage for a derivative order, supported by derivative symbol orders only.
    /// The value should be between 1 and 100 inclusive.
    /// The field is optional, if omitted the default leverage value of 10 will be used.
    #[serde(rename="lev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    leverage: Option<u32>,
    /// The trailing price for a trailing stop order
    price_trailing: Option<String>,
    /// Auxiliary Limit price (for STOP LIMIT)
    price_aux_limit: Option<String>,
    /// OCO stop price
    price_oco_stop: Option<String>,
    /// Time-In-Force: datetime for automatic order cancellation (ie. 2020-01-01 10:45:23) )
    #[serde(skip_serializing_if = "Option::is_none")]
    tif: Option<String>,
    /// The meta object allows you to pass along an affiliate code inside the object
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<OrderMeta>,
}

impl OrderForm {
    pub fn new(symbol: String, price: f64, amount: f64, order_type: OrderKind) -> Self {
        OrderForm {
            gid: None,
            cid: None,
            order_type,
            symbol,
            price: price.to_string(),
            amount: amount.to_string(),
            flags: Some(OrderFlag::default().bits),
            leverage: None,
            price_trailing: None,
            price_aux_limit: None,
            price_oco_stop: None,
            tif: None,
            meta: None,
        }
    }

    pub fn with_gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }

    pub fn with_cid(mut self, cid: u32) -> Self {
        self.cid = Some(cid);
        self
    }

    pub fn with_flags(mut self, flags: OrderFlag) -> Self {
        self.flags = Some(flags.bits());
        self
    }

    pub fn with_leverage(mut self, leverage: u32) -> Self {
        self.leverage = Some(leverage);
        self
    }

    pub fn with_price_trailing(mut self, trailing: f64) -> Result<Self, BoxError> {
        match self.order_type {
            OrderKind::TrailingStop => {
                self.price_trailing = Some(trailing.to_string());
                Ok(self)
            },
            _ => Err("Invalid order type.".into())
        }
    }

    pub fn with_price_aux_limit(mut self, limit: f64) -> Result<Self, BoxError> {
        match self.order_type {
            OrderKind::StopLimit | OrderKind::ExchangeStopLimit => {
                self.price_aux_limit = Some(limit.to_string());
                Ok(self)
            },
            _ => Err("Invalid order type.".into())
        }
    }

    pub fn with_price_oco_stop(mut self, oco_stop: f64) -> Result<Self, BoxError> {
        match self.flags {
            None => Err("No flags set.".into()),
            Some(flags) => {
                match OrderFlag::from_bits(flags) {
                    Some(flags) => {
                        if flags.contains(OrderFlag::OCO) {
                            self.price_oco_stop = Some(oco_stop.to_string());
                            return Ok(self);
                        }
                        return Err("OCO flag not set.".into());
                    },
                    None => Err("OCO flag not set.".into())
                }
            }
        }
    }

    pub fn with_tif<T: TimeZone>(mut self, tif: DateTime<T>) -> Self
        where T::Offset: Display {
        self.tif = Some(tif.format("%Y-%m-%d %H:%M:%S").to_string());
        self
    }

    pub fn with_meta(mut self, meta: OrderMeta) -> Self {
        self.meta = Some(meta);
        self
    }
}


#[derive(Clone)]
pub struct Orders {
    client: Client,
}

impl Orders {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Orders {
            client: Client::new(api_key, secret_key),
        }
    }

    pub async fn active_orders(&self) -> Result<Vec<ActiveOrder>, BoxError> {
        let payload: String = format!("{}", "{}");

        self.orders("orders".to_owned(), payload).await
    }

    pub async fn history<T>(&self, symbol: T) -> Result<Vec<ActiveOrder>, BoxError>
        where T: Into<Option<String>>
    {    
        let value = symbol.into().unwrap_or("".into());
        let payload: String = format!("{}", "{}");

        return if value.is_empty() {
            self.orders("orders/hist".into(), payload).await
        } else {
            let request: String = format!("orders/t{}/hist", value);
            self.orders(request, payload).await
        }
    }

    pub async fn orders<S>(&self, request: S, payload: S) -> Result<Vec<ActiveOrder>, BoxError>
        where S: Into<String>
    {
        let data = self.client.post_signed(request.into(), payload.into()).await?;

        let orders: Vec<ActiveOrder> = from_str(data.as_str())?;

        Ok(orders)
    }

    pub async fn submit_order(&self, order: &OrderForm) -> Result<ActiveOrder, BoxError> {
        let data = self.client.post_signed("order/submit".into(), serde_json::to_string(order)?).await?;

        let active_order = from_str(&data)?;

        Ok(active_order)
    }
}
