use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{Response, StatusCode};
use serde::Serialize;

use crate::auth;
use crate::endpoints::{AuthenticatedEndpoint, PublicEndpoint};
use crate::errors::RequestError;

static NO_PARAMS: &'static [(); 0] = &[];

#[derive(Clone, Debug)]
pub struct Client {
    api_key: String,
    secret_key: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Client {
            api_key: api_key.unwrap_or("".into()),
            secret_key: secret_key.unwrap_or("".into()),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, endpoint: PublicEndpoint) -> Result<String, RequestError> {
        let response = self.client.get(&endpoint.to_string()).send().await?;

        self.response_handler(response).await
    }

    pub async fn post_signed(
        &self,
        endpoint: &AuthenticatedEndpoint,
        payload: String,
    ) -> Result<String, RequestError> {
        self.post_signed_params(endpoint, payload, NO_PARAMS).await
    }

    pub async fn post_signed_params<P: Serialize + ?Sized>(
        &self,
        endpoint: &AuthenticatedEndpoint,
        payload: String,
        params: &P,
    ) -> Result<String, RequestError> {
        let response = self
            .client
            .post(&endpoint.to_string())
            .body(payload.clone())
            .query(params)
            .headers(self.build_headers(&endpoint, payload).await?)
            .send()
            .await?;

        self.response_handler(response).await
    }

    async fn build_headers(
        &self,
        endpoint: &AuthenticatedEndpoint,
        payload: String,
    ) -> Result<HeaderMap, RequestError> {
        let nonce = auth::generate_nonce().await;
        let path = endpoint.path();
        let signature_path = format!("/api{}{}{}", path, nonce, payload);

        let signature = auth::sign_payload(self.secret_key.as_bytes(), signature_path.as_bytes());

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("bitfinex-rs"));
        headers.insert(
            HeaderName::from_static("bfx-nonce"),
            HeaderValue::from_str(nonce.as_str()).map_err(RequestError::InvalidHeaderValue)?,
        );
        headers.insert(
            HeaderName::from_static("bfx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())
                .map_err(RequestError::InvalidHeaderValue)?,
        );
        headers.insert(
            HeaderName::from_static("bfx-signature"),
            HeaderValue::from_str(signature.as_str()).map_err(RequestError::InvalidHeaderValue)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(headers)
    }

    async fn response_handler(&self, response: Response) -> Result<String, RequestError> {
        match response.status() {
            StatusCode::OK => Ok(response.text().await.map_err(RequestError::MalformedText)?),
            StatusCode::INTERNAL_SERVER_ERROR => Err(RequestError::InternalServerError(
                response.text().await.map_err(RequestError::MalformedText)?,
            )),
            StatusCode::UNAUTHORIZED => Err(RequestError::Unauthorized),

            // StatusCode::SERVICE_UNAVAILABLE => {
            //     bail!("Service Unavailable: {}", response.text().await?);
            // }
            // StatusCode::BAD_REQUEST => {
            //     bail!(format!("Bad Request: {}", response.text().await?));
            // }
            _ => Err(RequestError::Unknown(
                response.text().await.map_err(RequestError::MalformedText)?,
            )),
        }
    }
}
