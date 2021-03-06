use std::fmt::Display;

use chrono::{DateTime, NaiveDate, TimeZone};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_str, Value};

use crate::client::Client;
use crate::endpoints::AuthenticatedEndpoint;
use crate::errors::BoxError;
use crate::responses::OrderResponse;

#[derive(Serialize, Clone, Debug)]
pub struct ActiveOrder {
    /// Order ID
    id: u64,
    /// Group ID
    group_id: Option<u64>,
    /// Client Order ID
    client_id: u64,
    /// Pair (tBTCUSD, …)
    symbol: String,
    /// Millisecond timestamp of creation
    creation_timestamp: u64,
    /// Millisecond timestamp of update
    update_timestamp: u64,
    /// Positive means buy, negative means sell.
    amount: f64,
    /// Original amount
    amount_original: f64,
    /// The type of the order: LIMIT, EXCHANGE LIMIT, MARKET, EXCHANGE MARKET, STOP, EXCHANGE STOP,
    /// STOP LIMIT, EXCHANGE STOP LIMIT, TRAILING STOP, EXCHANGE TRAILING STOP, FOK, EXCHANGE FOK,
    /// IOC, EXCHANGE IOC.
    order_type: OrderKind,
    /// Previous order type
    previous_order_type: Option<OrderKind>,
    /// Active flags for order
    flags: OrderFlags,
    /// Order Status: ACTIVE, PARTIALLY FILLED, RSN_PAUSE (trading is paused / paused due to AMPL rebase event)
    order_status: String,
    /// Price
    price: f64,
    /// Average price
    price_avg: Option<f64>,
    /// The trailing price
    price_trailing: Option<f64>,
    /// Auxiliary Limit price (for STOP LIMIT)
    price_aux_limit: Option<f64>,
    /// Set if order is hidden on the order book
    hidden: bool,
    /// If another order caused this order to be placed (OCO) this will be that other order's ID
    placed_id: Option<u64>,
    /// indicates origin of action: BFX, API>BFX
    routing: Option<String>,
    /// Additional meta information about the order ( $F7 = IS_POST_ONLY (0 if false, 1 if true),
    /// $F33 = Leverage (int), aff_code: "aff_code_here")
    meta: Option<String>,
}

impl ActiveOrder {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn group_id(&self) -> Option<u64> {
        self.group_id
    }
    pub fn client_id(&self) -> u64 {
        self.client_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn creation_timestamp(&self) -> u64 {
        self.creation_timestamp
    }
    pub fn update_timestamp(&self) -> u64 {
        self.update_timestamp
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn amount_original(&self) -> f64 {
        self.amount_original
    }
    pub fn order_type(&self) -> OrderKind {
        self.order_type
    }
    pub fn previous_order_type(&self) -> Option<OrderKind> {
        self.previous_order_type
    }
    pub fn flags(&self) -> OrderFlags {
        self.flags
    }
    pub fn order_status(&self) -> &str {
        &self.order_status
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn price_avg(&self) -> Option<f64> {
        self.price_avg
    }
    pub fn price_trailing(&self) -> Option<f64> {
        self.price_trailing
    }
    pub fn price_aux_limit(&self) -> Option<f64> {
        self.price_aux_limit
    }
    pub fn hidden(&self) -> bool {
        self.hidden
    }
    pub fn placed_id(&self) -> Option<u64> {
        self.placed_id
    }
    pub fn routing(&self) -> &Option<String> {
        &self.routing
    }
    pub fn meta(&self) -> &Option<String> {
        &self.meta
    }
}

impl<'de> Deserialize<'de> for ActiveOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let mut iterator = value
            .as_array()
            .ok_or(D::Error::custom("Missing main array"))?
            .iter();

        let id = iterator
            .next()
            .ok_or(D::Error::custom("Missing order id"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid order id type"))?;

        let gid = Option::deserialize(
            iterator
                .next()
                .ok_or(D::Error::custom("Missing order id"))?,
        )
        .map_err(D::Error::custom)?;

        let cid = iterator
            .next()
            .ok_or(D::Error::custom("Missing client id"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid client id type"))?;

        let symbol =
            String::deserialize(iterator.next().ok_or(D::Error::custom("Missing symbol"))?)
                .map_err(D::Error::custom)?;

        let mts_create = iterator
            .next()
            .ok_or(D::Error::custom("Missing mts create"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid mts create type"))?;

        let mts_update = iterator
            .next()
            .ok_or(D::Error::custom("Missing mts update"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid mts update type"))?;

        let amount = iterator
            .next()
            .ok_or(D::Error::custom("Missing amount"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid amount type"))?;

        let amount_orig = iterator
            .next()
            .ok_or(D::Error::custom("Missing amount orig"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid amount orig type"))?;

        let order_type = OrderKind::deserialize(
            iterator
                .next()
                .ok_or(D::Error::custom("Missing order type"))?,
        )
        .map_err(D::Error::custom)?;

        let prev_order_type = Option::deserialize(
            iterator
                .next()
                .ok_or(D::Error::custom("Missing prev order type"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholders
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        // sometimes the server returns a number, other times a string
        let flags = {
            let flags_value = iterator
                .next()
                .ok_or(D::Error::custom("Missing order flags"))?;

            if let Some(value) = flags_value.as_u64() {
                OrderFlags::from_bits(value as u32)
                    .ok_or(D::Error::custom("Invalid u64 flags value"))?
            } else if let Some(string) = flags_value.as_str() {
                OrderFlags::from_bits(string.parse().map_err(D::Error::custom)?)
                    .ok_or(D::Error::custom("Invalid string flags value"))?
            } else {
                return Err(D::Error::custom("Invalid flags value type"));
            }
        };

        let order_status = String::deserialize(
            iterator
                .next()
                .ok_or(D::Error::custom("Missing order type"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholders
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let price = iterator
            .next()
            .ok_or(D::Error::custom("Missing price"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid price type"))?;

        let price_avg = {
            let value = iterator
                .next()
                .ok_or(D::Error::custom("Missing price average"))?
                .as_f64()
                .ok_or(D::Error::custom("Invalid price average type"))?;

            if value > 0.0 {
                Some(value)
            } else {
                None
            }
        };

        let price_trailing = {
            let value = iterator
                .next()
                .ok_or(D::Error::custom("Missing trailing price"))?
                .as_f64()
                .ok_or(D::Error::custom("Invalid trailing price"))?;

            if value > 0.0 {
                Some(value)
            } else {
                None
            }
        };

        let price_aux_limit = {
            let value = iterator
                .next()
                .ok_or(D::Error::custom("Missing price aux limit"))?
                .as_f64()
                .ok_or(D::Error::custom("Invalid price aux limit"))?;

            if value > 0.0 {
                Some(value)
            } else {
                None
            }
        };

        // skip placeholders
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let hidden = iterator
            .next()
            .ok_or(D::Error::custom("Missing hidden"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid hidden type"))?
            > 0;

        let placed_id = Option::deserialize(
            iterator
                .next()
                .ok_or(D::Error::custom("Missing placed_id"))?,
        )
        .map_err(D::Error::custom)?;

        let routing =
            Option::deserialize(iterator.next().ok_or(D::Error::custom("Missing routing"))?)
                .map_err(D::Error::custom)?;

        // skip placeholders
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        iterator
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        // TODO: define specialized struct
        let meta = Option::deserialize(iterator.next().ok_or(D::Error::custom("Missing meta"))?)
            .map_err(D::Error::custom)?;

        Ok(Self {
            id,
            group_id: gid,
            client_id: cid,
            symbol,
            creation_timestamp: mts_create,
            update_timestamp: mts_update,
            amount,
            amount_original: amount_orig,
            order_type,
            previous_order_type: prev_order_type,
            flags,
            order_status,
            price,
            price_avg,
            price_trailing,
            price_aux_limit,
            hidden,
            placed_id,
            routing,
            meta,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum OrderKind {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "EXCHANGE LIMIT")]
    ExchangeLimit,
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "EXCHANGE MARKET")]
    ExchangeMarket,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "EXCHANGE STOP")]
    ExchangeStop,
    #[serde(rename = "STOP LIMIT")]
    StopLimit,
    #[serde(rename = "EXCHANGE STOP LIMIT")]
    ExchangeStopLimit,
    #[serde(rename = "TRAILING STOP")]
    TrailingStop,
    #[serde(rename = "EXCHANGE TRAILING STOP")]
    ExchangeTrailingStop,
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "EXCHANGE FOK")]
    ExchangeFok,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "EXCHANGE IOC")]
    ExchangeIoc,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderMeta {
    aff_code: String,
}

impl OrderMeta {
    pub fn new(aff_code: String) -> Self {
        OrderMeta { aff_code }
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct OrderFlags: u32 {
        const NONE = 0;
        const HIDDEN = 64;
        const CLOSE = 512;
        const REDUCE_ONLY = 1024;
        const POST_ONLY = 4096;
        const OCO = 16384;
        const NO_VAR_RATES = 524288;
    }
}

#[derive(Serialize, Clone, Debug)]
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
    #[serde(rename = "type")]
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
    #[serde(rename = "lev")]
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
            flags: Some(OrderFlags::NONE.bits),
            leverage: None,
            price_trailing: None,
            price_aux_limit: None,
            price_oco_stop: None,
            tif: None,
            meta: None,
        }
    }

    pub fn with_gid(mut self, gid: Option<u32>) -> Self {
        self.gid = gid;
        self
    }

    pub fn with_cid(mut self, cid: Option<u32>) -> Self {
        self.cid = cid;
        self
    }

    pub fn with_flags(mut self, flags: Option<OrderFlags>) -> Self {
        self.flags = flags.map(|x| x.bits());
        self
    }

    pub fn with_leverage(mut self, leverage: Option<u32>) -> Self {
        self.leverage = leverage;
        self
    }

    pub fn with_price_trailing(mut self, trailing: Option<f64>) -> Result<Self, BoxError> {
        match self.order_type {
            OrderKind::TrailingStop => {
                self.price_trailing = trailing.map(|x| x.to_string());
                Ok(self)
            }
            _ => Err("Invalid order type.".into()),
        }
    }

    pub fn with_price_aux_limit(mut self, limit: Option<f64>) -> Result<Self, BoxError> {
        match self.order_type {
            OrderKind::StopLimit | OrderKind::ExchangeStopLimit => {
                self.price_aux_limit = limit.map(|x| x.to_string());
                Ok(self)
            }
            _ => Err("Invalid order type.".into()),
        }
    }

    pub fn with_price_oco_stop(mut self, oco_stop: Option<f64>) -> Result<Self, BoxError> {
        match self.flags {
            None => Err("No flags set.".into()),
            Some(flags) => match OrderFlags::from_bits(flags) {
                Some(flags) => {
                    if flags.contains(OrderFlags::OCO) {
                        self.price_oco_stop = oco_stop.map(|x| x.to_string());
                        return Ok(self);
                    }
                    return Err("OCO flag not set.".into());
                }
                None => Err("OCO flag not set.".into()),
            },
        }
    }

    pub fn with_tif<T: TimeZone>(mut self, tif: Option<DateTime<T>>) -> Self
    where
        T::Offset: Display,
    {
        self.tif = tif.map(|x| x.format("%Y-%m-%d %H:%M:%S").to_string());
        self
    }

    pub fn with_meta(mut self, meta: Option<OrderMeta>) -> Self {
        self.meta = meta;
        self
    }
}

#[derive(Serialize)]
pub struct CancelOrderForm {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    client_id: Option<u64>,
    #[serde(rename = "cid_date", skip_serializing_if = "Option::is_none")]
    client_id_date: Option<CancelOrderDateTime>,
}

impl CancelOrderForm {
    pub fn from_id(id: u64) -> Self {
        CancelOrderForm {
            id: Some(id),
            client_id: None,
            client_id_date: None,
        }
    }

    pub fn from_client<Tz: TimeZone>(client_id: u64, client_id_date: DateTime<Tz>) -> Self {
        let naive_date = client_id_date.naive_utc().date();

        CancelOrderForm {
            id: None,
            client_id: Some(client_id),
            client_id_date: Some(CancelOrderDateTime { date: naive_date }),
        }
    }
}

struct CancelOrderDateTime {
    date: NaiveDate,
}

impl Serialize for CancelOrderDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.date.format("%Y-%m-%d").to_string())
    }
}

impl From<ActiveOrder> for CancelOrderForm {
    fn from(o: ActiveOrder) -> Self {
        Self::from_id(o.id)
    }
}

#[derive(Clone)]
pub struct Orders {
    client: Client,
}

impl Orders {
    pub fn new(client: Client) -> Self {
        Orders { client }
    }

    pub async fn active_orders(&self) -> Result<Vec<ActiveOrder>, BoxError> {
        let endpoint = AuthenticatedEndpoint::RetrieveOrders;
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }

    pub async fn history<S: ToString>(
        &self,
        symbol: Option<S>,
    ) -> Result<Vec<ActiveOrder>, BoxError> {
        let endpoint = match symbol {
            Some(symbol) => AuthenticatedEndpoint::OrdersHistory {
                symbol: Some(symbol.to_string()),
            },
            None => AuthenticatedEndpoint::OrdersHistory { symbol: None },
        };

        println!("Endpoint: {:?}", endpoint);
        let data = self.client.post_signed(&endpoint, "{}".into()).await?;

        Ok(from_str(&data)?)
    }

    pub async fn submit_order(&self, order: &OrderForm) -> Result<OrderResponse, BoxError> {
        let endpoint = AuthenticatedEndpoint::SubmitOrder;
        let data = self
            .client
            .post_signed(&endpoint, serde_json::to_string(order)?)
            .await?;

        Ok(from_str(&data)?)
    }

    pub async fn cancel_order(
        &self,
        order_form: &CancelOrderForm,
    ) -> Result<OrderResponse, BoxError> {
        let endpoint = AuthenticatedEndpoint::CancelOrder;

        let data = self
            .client
            .post_signed(&endpoint, serde_json::to_string(order_form)?)
            .await?;

        Ok(from_str(&data)?)
    }
}
