#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tungstenite;
extern crate url;

mod auth;
mod client;
mod endpoints;
mod ledger;
mod trades;

pub mod account;
pub mod api;
pub mod book;
pub mod candles;
pub mod currency;
pub mod errors;
pub mod events;
pub mod orders;
pub mod pairs;
pub mod positions;
pub mod responses;
pub mod ticker;
pub mod websockets;
