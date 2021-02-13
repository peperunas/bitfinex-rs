use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::account::WalletKind;
use crate::orders::{OrderFlags, OrderKind};

#[derive(Deserialize, Debug)]
pub enum ResponseStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "FAILUR")]
    Failure,
}
#[derive(Deserialize, Debug)]
pub enum WalletTransferResponseKind {
    #[serde(rename = "acc_tf")]
    AccountTransfer,
}

#[derive(Debug)]
pub struct WalletTransferResponse {
    /// Millisecond Time Stamp of the update
    mts: u64,
    response_type: WalletTransferResponseKind,
    /// unique ID of the message
    id: Option<u64>,
    /// Millisecond Time Stamp when the transfer was created
    mts_update: u64,
    /// Starting wallet
    from: WalletKind,
    /// Destination wallet
    to: WalletKind,
    /// Currency
    symbol_from: String,
    /// Destination currency
    symbol_to: Option<String>,
    /// Amount of Transfer
    amount: f64,
    /// Status of the notification; it may vary over time (SUCCESS, ERROR, FAILURE, ...)
    status: ResponseStatus,
    /// Text of the notification
    text: String,
}

impl WalletTransferResponse {
    pub fn mts(&self) -> u64 {
        self.mts
    }
    pub fn response_type(&self) -> &WalletTransferResponseKind {
        &self.response_type
    }
    pub fn id(&self) -> Option<u64> {
        self.id
    }
    pub fn mts_update(&self) -> u64 {
        self.mts_update
    }
    pub fn from(&self) -> &WalletKind {
        &self.from
    }
    pub fn to(&self) -> &WalletKind {
        &self.to
    }
    pub fn symbol_from(&self) -> &str {
        &self.symbol_from
    }
    pub fn symbol_to(&self) -> &Option<String> {
        &self.symbol_to
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn status(&self) -> &ResponseStatus {
        &self.status
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl<'de> Deserialize<'de> for WalletTransferResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        // this can either be an array within an array
        // OR an array on itself
        let mut middle_list_iter = {
            let middle_list = value
                .get(4)
                .ok_or(D::Error::custom("Missing central array"))?
                .as_array()
                .ok_or(D::Error::custom("Invalid central array"))?;

            // accessing the inner list
            // checking if it is an array on its own or not
            match middle_list
                .iter()
                .next()
                .ok_or(D::Error::custom("Empty central array"))?
                .as_array()
            {
                Some(array) => array.into_iter(),
                None => middle_list.into_iter(),
            }
        };

        let mts = value
            .get(0)
            .ok_or(D::Error::custom("Missing mts"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid mts value"))?
            .round() as u64;

        let response_type = WalletTransferResponseKind::deserialize(
            value
                .get(1)
                .ok_or(D::Error::custom("Missing response type"))?,
        )
        .map_err(D::Error::custom)?;

        let message_id =
            Option::deserialize(value.get(2).ok_or(D::Error::custom("Missing message_id"))?)
                .map_err(D::Error::custom)?;

        let mts_update = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing mts update"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid mts update type"))?;

        let wallet_from = WalletKind::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing source wallet"))?,
        )
        .map_err(D::Error::custom)?;

        let wallet_to = WalletKind::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing destination wallet"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholder
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let symbol_from = String::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing source currency"))?,
        )
        .map_err(D::Error::custom)?;

        let symbol_to = Option::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing destination currency"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholder
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let amount = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing amount"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid amount type"))?;

        let status =
            ResponseStatus::deserialize(value.get(6).ok_or(D::Error::custom("Missing status"))?)
                .map_err(D::Error::custom)?;

        let text = String::deserialize(value.get(7).ok_or(D::Error::custom("Missing text"))?)
            .map_err(D::Error::custom)?;

        Ok(Self {
            mts,
            response_type,
            id: message_id,
            mts_update,
            from: wallet_from,
            to: wallet_to,
            symbol_from,
            symbol_to,
            amount,
            status,
            text,
        })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct OrderResponse {
    /// Millisecond Time Stamp of the update
    mts: u64,
    /// Purpose of notification ('on-req', 'oc-req', 'uca', 'fon-req', 'foc-req')
    #[serde(rename(serialize = "type"))]
    response_type: OrderResponseKind,
    /// unique ID of the message
    message_id: Option<u64>,
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
    /// STOP LIMIT, EXCHANGE STOP LIMIT, TRAILING STOP, EXCHANGE TRAILING STOP,
    /// FOK, EXCHANGE FOK, IOC, EXCHANGE IOC.
    order_type: OrderKind,
    /// Previous order type
    previous_order_type: Option<OrderKind>,
    /// Millisecond timestamp of Time-In-Force: automatic order cancellation
    mts_tif: Option<u64>,
    /// Order Status: ACTIVE, EXECUTED @ PRICE(AMOUNT) e.g. "EXECUTED @ 107.6(-0.2)",
    /// PARTIALLY FILLED @ PRICE(AMOUNT), INSUFFICIENT MARGIN was: PARTIALLY FILLED @ PRICE(AMOUNT),
    /// CANCELED, CANCELED was: PARTIALLY FILLED @ PRICE(AMOUNT),
    /// RSN_DUST (amount is less than 0.00000001),
    /// RSN_PAUSE (trading is paused / paused due to AMPL rebase event)
    order_status: String,
    /// Price
    price: f64,
    /// Average price
    price_avg: Option<f64>,
    /// The trailing price
    price_trailing: Option<f64>,
    /// Auxiliary Limit price (for STOP LIMIT)
    price_aux_limit: Option<f64>,
    /// Flag is order is hidden
    hidden: bool,
    /// If another order caused this order to be placed (OCO) this will be that other order's ID
    placed_id: Option<u64>,
    /// indicates origin of action: BFX, ETHFX, API>BFX, API>ETHFX
    routing: String,
    /// See https://docs.bitfinex.com/v2/docs/flag-values.
    flags: OrderFlags,
    /// Additional meta information about the order ( $F7 = IS_POST_ONLY (0 if false, 1 if true), $F33 = Leverage (int))
    meta: Option<String>,
    /// Work in progress
    code: u64,
    /// Status of the notification; it may vary over time (SUCCESS, ERROR, FAILURE, ...)
    status: String,
    /// Text of the notification
    text: String,
}

impl OrderResponse {
    pub fn mts(&self) -> u64 {
        self.mts
    }
    pub fn response_type(&self) -> &OrderResponseKind {
        &self.response_type
    }
    pub fn message_id(&self) -> Option<u64> {
        self.message_id
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn gid(&self) -> Option<u64> {
        self.group_id
    }
    pub fn cid(&self) -> u64 {
        self.client_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn mts_create(&self) -> u64 {
        self.creation_timestamp
    }
    pub fn mts_update(&self) -> u64 {
        self.update_timestamp
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn amount_orig(&self) -> f64 {
        self.amount_original
    }
    pub fn order_type(&self) -> OrderKind {
        self.order_type
    }
    pub fn prev_order_type(&self) -> Option<OrderKind> {
        self.previous_order_type
    }
    pub fn mts_tif(&self) -> Option<u64> {
        self.mts_tif
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
    pub fn routing(&self) -> &str {
        &self.routing
    }
    pub fn flags(&self) -> OrderFlags {
        self.flags
    }
    pub fn meta(&self) -> &Option<String> {
        &self.meta
    }
    pub fn code(&self) -> u64 {
        self.code
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl<'de> Deserialize<'de> for OrderResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        // this can either be an array within an array
        // OR an array on itself
        let mut middle_list_iter = {
            let middle_list = value
                .get(4)
                .ok_or(D::Error::custom("Missing central array"))?
                .as_array()
                .ok_or(D::Error::custom("Invalid central array"))?;

            // accessing the inner list
            // checking if it is an array on its own or not
            match middle_list
                .iter()
                .next()
                .ok_or(D::Error::custom("Empty central array"))?
                .as_array()
            {
                Some(array) => array.into_iter(),
                None => middle_list.into_iter(),
            }
        };

        let mts = value
            .get(0)
            .ok_or(D::Error::custom("Missing mts"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid mts value"))?
            .round() as u64;

        let response_type = OrderResponseKind::deserialize(
            value
                .get(1)
                .ok_or(D::Error::custom("Missing response type"))?,
        )
        .map_err(D::Error::custom)?;

        let message_id =
            Option::deserialize(value.get(2).ok_or(D::Error::custom("Missing message_id"))?)
                .map_err(D::Error::custom)?;

        let id = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing order id"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid order id type"))?;

        let gid = Option::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing order id"))?,
        )
        .map_err(D::Error::custom)?;

        let cid = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing client id"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid client id type"))?;

        let symbol = String::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing symbol"))?,
        )
        .map_err(D::Error::custom)?;

        let mts_create = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing mts create"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid mts create type"))?;

        let mts_update = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing mts update"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid mts update type"))?;

        let amount = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing amount"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid amount type"))?;

        let amount_orig = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing amount orig"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid amount orig type"))?;

        let order_type = OrderKind::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing order type"))?,
        )
        .map_err(D::Error::custom)?;

        let prev_order_type = Option::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing prev order type"))?,
        )
        .map_err(D::Error::custom)?;

        let mts_tif = Option::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing mts_tif"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholder
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let flags = OrderFlags::from_bits(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing order flags"))?
                .as_u64()
                .ok_or(D::Error::custom("Invalid order flags type"))? as u32,
        )
        .ok_or(D::Error::custom("Invalid order flags"))?;

        let order_status = String::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing order type"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholders
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let price = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing price"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid price type"))?;

        let price_avg = {
            let value = middle_list_iter
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
            let value = middle_list_iter
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
            let value = middle_list_iter
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
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let hidden = middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing hidden"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid hidden type"))?
            > 0;

        let placed_id = Option::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing placed_id"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholders
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        let routing = String::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing routing"))?,
        )
        .map_err(D::Error::custom)?;

        // skip placeholder
        middle_list_iter
            .next()
            .ok_or(D::Error::custom("Missing placeholder"))?;

        // TODO: define specialized struct
        let meta = Option::deserialize(
            middle_list_iter
                .next()
                .ok_or(D::Error::custom("Missing meta"))?,
        )
        .map_err(D::Error::custom)?;

        let status = String::deserialize(value.get(6).ok_or(D::Error::custom("Missing status"))?)
            .map_err(D::Error::custom)?;

        let text = String::deserialize(value.get(7).ok_or(D::Error::custom("Missing text"))?)
            .map_err(D::Error::custom)?;

        Ok(Self {
            mts,
            response_type,
            message_id,
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
            mts_tif,
            order_status,
            price,
            price_avg,
            price_trailing,
            price_aux_limit,
            hidden,
            placed_id,
            routing,
            flags,
            meta,
            code: 0,
            status,
            text,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderResponseKind {
    #[serde(rename = "on-req")]
    NewOrderRequest,
    #[serde(rename = "oc-req")]
    CancelOrderRequest,
    #[serde(rename = "uca")]
    Uca,
    #[serde(rename = "fon-req")]
    FundingNewOrderRequest,
    #[serde(rename = "foc-req")]
    FundingCancelOrderRequest,
}

#[derive(Serialize, Debug)]
pub struct TradeResponse {
    trade_id: u64,
    symbol: String,
    execution_timestamp: u64,
    order_id: u64,
    execution_amount: f64,
    execution_price: f64,
    order_type: Option<OrderKind>,
    order_price: Option<f64>,
    is_maker: bool,
    fee: f64,
    fee_currency: String,
}

impl TradeResponse {
    pub fn trade_id(&self) -> u64 {
        self.trade_id
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn execution_timestamp(&self) -> u64 {
        self.execution_timestamp
    }
    pub fn order_id(&self) -> u64 {
        self.order_id
    }
    pub fn execution_amount(&self) -> f64 {
        self.execution_amount
    }
    pub fn execution_price(&self) -> f64 {
        self.execution_price
    }
    pub fn order_type(&self) -> Option<OrderKind> {
        self.order_type
    }
    pub fn order_price(&self) -> Option<f64> {
        self.order_price
    }
    pub fn is_maker(&self) -> bool {
        self.is_maker
    }
    pub fn fee(&self) -> f64 {
        self.fee
    }
    pub fn fee_currency(&self) -> &str {
        &self.fee_currency
    }
}

impl<'de> Deserialize<'de> for TradeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?
            .as_array()
            .ok_or(D::Error::custom("Invalid trade array"))?
            .to_owned();
        let mut iterator = value.iter();

        let trade_id = iterator
            .next()
            .ok_or(D::Error::custom("Missing trade id"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid trade id"))?;

        let symbol = iterator
            .next()
            .ok_or(D::Error::custom("Missing symbol"))?
            .as_str()
            .ok_or(D::Error::custom("Invalid symbol"))?
            .into();

        let execution_timestamp = iterator
            .next()
            .ok_or(D::Error::custom("Missing execution timestamp"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid execution timestamp"))?;

        let order_id = iterator
            .next()
            .ok_or(D::Error::custom("Missing order id"))?
            .as_u64()
            .ok_or(D::Error::custom("Invalid order id"))?;

        let execution_amount = iterator
            .next()
            .ok_or(D::Error::custom("Missing execution amount"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid execution amount"))?;

        let execution_price = iterator
            .next()
            .ok_or(D::Error::custom("Missing execution price"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid execution price"))?;

        let order_type = Option::deserialize(
            iterator
                .next()
                .ok_or(D::Error::custom("Missing order type"))?,
        )
        .map_err(D::Error::custom)?;

        let order_price = Option::deserialize(
            iterator
                .next()
                .ok_or(D::Error::custom("Missing order price"))?,
        )
        .map_err(D::Error::custom)?;

        let is_maker = iterator
            .next()
            .ok_or(D::Error::custom("Missing maker fee"))?
            .as_i64()
            .ok_or(D::Error::custom("Invalid hidden fee"))?
            > 0;

        let fee = iterator
            .next()
            .ok_or(D::Error::custom("Missing fee"))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid fee amount"))?;

        let fee_currency = iterator
            .next()
            .ok_or(D::Error::custom("Missing fee currency"))?
            .as_str()
            .ok_or(D::Error::custom("Invalid fee currency"))?
            .into();

        Ok(Self {
            trade_id,
            symbol,
            execution_timestamp,
            order_id,
            execution_amount,
            execution_price,
            order_type,
            order_price,
            is_maker,
            fee,
            fee_currency,
        })
    }
}

pub struct AccountFees {
    // Shows the maker fee rate for the account
    maker_fee: f64,
    // Shows the maker rebate for derivative trades on the account
    derivative_rebate: f64,
    // Shows the taker fee rate for crypto to crypto trades on the account
    taker_to_crypto: f64,
    // Shows the taker fee rate for crypto to stable coin trades on the account
    taker_to_stable: f64,
    // Shows the taker fee rate for crypto to stable coin trades on the account
    taker_to_fiat: f64,
    // Shows the taker fee rate for derivative trades on the account
    derivative_taker: f64,
}

impl AccountFees {
    pub fn maker_fee(&self) -> f64 {
        self.maker_fee
    }
    pub fn derivative_rebate(&self) -> f64 {
        self.derivative_rebate
    }
    pub fn taker_to_crypto(&self) -> f64 {
        self.taker_to_crypto
    }
    pub fn taker_to_stable(&self) -> f64 {
        self.taker_to_stable
    }
    pub fn taker_to_fiat(&self) -> f64 {
        self.taker_to_fiat
    }
    pub fn derivative_taker(&self) -> f64 {
        self.derivative_taker
    }
}

impl<'de> Deserialize<'de> for AccountFees {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?
            .as_array()
            .ok_or(D::Error::custom("Invalid response"))?
            .to_owned();

        let taker_maker_arrays = value
            .get(4)
            .ok_or(D::Error::custom("Missing inner arrays."))?
            .as_array()
            .ok_or(D::Error::custom("Invalid inner arrays"))?;

        let maker_array = taker_maker_arrays
            .get(0)
            .ok_or(D::Error::custom("Missing maker fees array."))?
            .as_array()
            .ok_or(D::Error::custom("Invalid maker fees array"))?;

        let taker_array = taker_maker_arrays
            .get(1)
            .ok_or(D::Error::custom("Missing taker fees array."))?
            .as_array()
            .ok_or(D::Error::custom("Invalid taker fees array"))?;

        let maker_fee = maker_array
            .get(0)
            .ok_or(D::Error::custom("Missing maker fee."))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid maker fee value."))?;

        let derivative_rebate = maker_array
            .get(5)
            .ok_or(D::Error::custom("Missing derivative rebate."))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid derivative rebate value."))?;

        let taker_to_crypto = taker_array
            .get(0)
            .ok_or(D::Error::custom("Missing maker fee."))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid maker fee value."))?;

        let taker_to_stable = taker_array
            .get(1)
            .ok_or(D::Error::custom("Missing derivative rebate."))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid derivative rebate value."))?;

        let taker_to_fiat = taker_array
            .get(2)
            .ok_or(D::Error::custom("Missing derivative rebate."))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid derivative rebate value."))?;

        let derivative_taker = taker_array
            .get(5)
            .ok_or(D::Error::custom("Missing derivative rebate."))?
            .as_f64()
            .ok_or(D::Error::custom("Invalid derivative rebate value."))?;

        Ok(Self {
            maker_fee,
            derivative_rebate,
            taker_to_crypto,
            taker_to_stable,
            taker_to_fiat,
            derivative_taker,
        })
    }
}
