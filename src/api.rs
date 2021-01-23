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
        Bitfinex {
            book: Book::new(),
            ticker: Ticker::new(),
            trades: Trades::new(),
            candles: Candles::new(),
            orders: Orders::new(api_key.clone(), secret_key.clone()),
            account: Account::new(api_key.clone(), secret_key.clone()),
            ledger: Ledger::new(api_key.clone(), secret_key.clone()),
            positions: Positions::new(api_key.clone(), secret_key.clone()),
        }
    }
}
