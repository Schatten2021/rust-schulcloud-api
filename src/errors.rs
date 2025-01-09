use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Errors {
    RequestError(reqwest::Error),
    NotJsonError(reqwest::Error),
    JsonDeserializeError(serde_json::Error),
    APIError(String, String, String),
    EncryptionError(openssl::error::ErrorStack),
    ValueError(String),
    Base64Error(base64::DecodeError),
    HexError(hex::FromHexError),
    StringDecodeError(FromUtf8Error),
    OtherErrors(String),
    NotAuthenticated,
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Errors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Errors::RequestError(e) => Some(e),
            Errors::NotJsonError(e) => Some(e),
            Errors::JsonDeserializeError(e) => Some(e),
            Errors::APIError(_, _, _) => None,
            Errors::EncryptionError(e) => Some(e),
            Errors::ValueError(_) => None,
            Errors::Base64Error(e) => Some(e),
            Errors::HexError(e) => Some(e),
            Errors::StringDecodeError(e) => Some(e),
            Errors::OtherErrors(_) => None,
            Errors::NotAuthenticated => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Errors>;