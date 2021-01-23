use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{Response, StatusCode};
use serde::Serialize;

use crate::auth;
use crate::endpoints::{AuthenticatedEndpoint, PublicEndpoint};
use crate::errors::BoxError;

static NO_PARAMS: &'static [(); 0] = &[];

#[derive(Clone, Debug)]
pub struct Client {
    api_key: String,
    secret_key: String,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Client {
            api_key: api_key.unwrap_or("".into()),
            secret_key: secret_key.unwrap_or("".into()),
        }
    }

    pub async fn get(&self, endpoint: PublicEndpoint) -> Result<String, BoxError> {
        let response = reqwest::get(&endpoint.to_string()).await?;

        self.handler(response).await
    }

    pub async fn post_signed(
        &self,
        endpoint: &AuthenticatedEndpoint,
        payload: String,
    ) -> Result<String, BoxError> {
        self.post_signed_params(endpoint, payload, NO_PARAMS).await
    }

    pub async fn post_signed_params<P: Serialize + ?Sized>(
        &self,
        endpoint: &AuthenticatedEndpoint,
        payload: String,
        params: &P,
    ) -> Result<String, BoxError> {
        let client = reqwest::Client::new();

        let response = client
            .post(&endpoint.to_string())
            .headers(self.build_headers(&endpoint, payload.clone())?)
            .body(payload)
            .query(params)
            .send()
            .await?;

        self.handler(response).await
    }

    fn build_headers(
        &self,
        endpoint: &AuthenticatedEndpoint,
        payload: String,
    ) -> Result<HeaderMap, BoxError> {
        let nonce: String = auth::generate_nonce()?;
        let path = endpoint
            .to_string()
            .strip_prefix(AuthenticatedEndpoint::HOST)
            .ok_or("Invalid endpoint")?
            .to_owned();
        let signature_path = format!("/api{}{}{}", path, nonce, payload);

        let signature = auth::sign_payload(self.secret_key.as_bytes(), signature_path.as_bytes())?;

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("bitfinex-rs"));
        headers.insert(
            HeaderName::from_static("bfx-nonce"),
            HeaderValue::from_str(nonce.as_str())?,
        );
        headers.insert(
            HeaderName::from_static("bfx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );
        headers.insert(
            HeaderName::from_static("bfx-signature"),
            HeaderValue::from_str(signature.as_str())?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(headers)
    }

    async fn handler(&self, response: Response) -> Result<String, BoxError> {
        match response.status() {
            StatusCode::OK => {
                let body = response.text().await?;

                return Ok(body);
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error: {}", response.text().await?);
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable: {}", response.text().await?);
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized: {}", response.text().await?);
            }
            StatusCode::BAD_REQUEST => {
                bail!(format!("Bad Request: {}", response.text().await?));
            }
            s => {
                bail!(format!(
                    "Received response {}: {}",
                    s,
                    response.text().await?
                ));
            }
        };
    }
}
