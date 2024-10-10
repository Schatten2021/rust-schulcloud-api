use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Errors {
    RequestError(reqwest::Error),
    NotJsonError(reqwest::Error),
    JsonDeserializeError(serde_json::Error),
    APIError(String, String),
    EncryptionError(openssl::error::ErrorStack),
    ValueError(String),
    Base64Error(base64::DecodeError),
    HexError(hex::FromHexError),
    StringDecodeError(FromUtf8Error),
    OtherErrors(String),
}