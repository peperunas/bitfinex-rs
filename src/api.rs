use crate::account::Account;
use crate::book::Book;
use crate::candles::Candles;
use crate::ledger::Ledger;
use crate::orders::Orders;
use crate::positions::Positions;
use crate::ticker::Ticker;
use crate::trades::Trades;

#[derive(Clone)]
pub struct Bitfinex {
    pub book: Book,
    pub ticker: Ticker,
    pub trades: Trades,
    pub candles: Candles,
    pub orders: Orders,
    pub account: Account,
    pub ledger: Ledger,
    pub positions: Positions,
}

impl Bitfinex {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        let client = crate::client::Client::new(api_key.clone(), secret_key.clone());

        Bitfinex {
            book: Book::new(client.clone()),
            ticker: Ticker::new(client.clone()),
            trades: Trades::new(client.clone()),
            candles: Candles::new(client.clone()),
            orders: Orders::new(client.clone()),
            account: Account::new(client.clone()),
            ledger: Ledger::new(client.clone()),
            positions: Positions::new(client.clone()),
        }
    }
}
