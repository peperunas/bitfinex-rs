use std::time::{SystemTime, UNIX_EPOCH};

use hex::encode;
use ring::hmac;

use crate::errors::BoxError;
use bitflags::_core::time::Duration;
use rand::Rng;

pub fn sign_payload(secret: &[u8], payload: &[u8]) -> Result<String, BoxError> {
    let signed_key = hmac::Key::new(hmac::HMAC_SHA384, secret);
    let signature = encode(hmac::sign(&signed_key, payload).as_ref());

    Ok(signature)
}

pub async fn generate_nonce() -> Result<String, BoxError> {
    let random_sleep_duration: u8 = rand::thread_rng().gen();
    tokio::time::delay_for(Duration::from_nanos(random_sleep_duration as u64)).await;

    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let timestamp = since_epoch.as_micros();

    Ok((timestamp.to_string()))
}
