
use serde_json::from_str;
use crate::client::Client;
use crate::errors::BoxError;

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

#[derive(Clone)]
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

        if value.is_empty() {
            return self.orders("orders/hist".into(), payload).await;
        } else {
            let request: String = format!("orders/t{}/hist", value);
            return self.orders(request, payload).await;
        }
    }

    pub async fn orders<S>(&self, request: S, payload: S) -> Result<Vec<ActiveOrder>, BoxError>
        where S: Into<String>
    {
        let data = self.client.post_signed(request.into(), payload.into()).await?;

        let orders: Vec<ActiveOrder> = from_str(data.as_str())?;

        Ok(orders)
    }
}