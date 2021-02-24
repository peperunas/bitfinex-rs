use thiserror::Error;

// use std;
//
// use reqwest;
// use serde_json;
// use tungstenite;
// use url;

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Invalid request")]
    InvalidRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Unauthorized request")]
    Unauthorized,
    #[error("Malformed response text")]
    MalformedText(#[from] reqwest::Error),
    #[error("Unknown error: {0}")]
    Unknown(String),
}
// error_chain! {
//     types {
//         Error, ErrorKind, ResultExt, Result;
//     }
//
//     errors {
//         Internal(t: String) {
//             description("invalid toolchain name")
//             display("invalid toolchain name: '{}'", t)
//         }
//     }
//
//     foreign_links {
//         ReqError(reqwest::Error);
//         InvalidHeaderError(reqwest::header::InvalidHeaderValue);
//         IoError(std::io::Error);
//         ParseFloatError(std::num::ParseFloatError);
//         UrlParserError(url::ParseError);
//         Json(serde_json::Error);
//         Tungstenite(tungstenite::Error);
//         TimestampError(std::time::SystemTimeError);
//     }
//
// }
