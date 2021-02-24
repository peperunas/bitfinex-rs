use std::time::{SystemTime, UNIX_EPOCH};

use bitflags::_core::time::Duration;
use hex::encode;
use rand::Rng;
use ring::hmac;

pub fn sign_payload(secret: &[u8], payload: &[u8]) -> String {
    let signed_key = hmac::Key::new(hmac::HMAC_SHA384, secret);
    let signature = encode(hmac::sign(&signed_key, payload).as_ref());

    signature
}

pub async fn generate_nonce() -> String {
    // provides different values on concurrent requests
    let random_sleep_duration: u8 = rand::thread_rng().gen();
    tokio::time::sleep(Duration::from_nanos(random_sleep_duration as u64)).await;

    // unwrapping since duration_since returns an error if `earlier` is later than self.
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = since_epoch.as_micros();

    timestamp.to_string()
}
