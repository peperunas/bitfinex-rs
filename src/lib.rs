#![deny(
unstable_features,
unused_must_use,
unused_mut,
unused_imports,
unused_import_braces)]

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

mod client;
mod trades;
mod account;
mod ledger;
mod auth;
mod endpoints;

pub mod candles;
pub mod api;
pub mod pairs;
pub mod currency;
pub mod websockets;
pub mod events;
pub mod errors;
pub mod positions;
pub mod ticker;
pub mod orders;
pub mod book;

