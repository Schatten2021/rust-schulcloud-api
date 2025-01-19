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
impl From<reqwest::Error> for Errors {
    fn from(e: reqwest::Error) -> Self {
        Errors::RequestError(e)
    }
}
impl From<serde_json::Error> for Errors {
    fn from(e: serde_json::Error) -> Self {
        Errors::JsonDeserializeError(e)
    }
}
impl From<openssl::error::ErrorStack> for Errors {
    fn from(e: openssl::error::ErrorStack) -> Self {
        Errors::EncryptionError(e)
    }
}
impl From<base64::DecodeError> for Errors {
    fn from(e: base64::DecodeError) -> Self {
        Errors::Base64Error(e)
    }
}
impl From<hex::FromHexError> for Errors {
    fn from(e: hex::FromHexError) -> Self {
        Errors::HexError(e)
    }
}
impl From<FromUtf8Error> for Errors {
    fn from(e: FromUtf8Error) -> Self {
        Errors::StringDecodeError(e)
    }
}
pub type Result<T> = std::result::Result<T, Errors>;