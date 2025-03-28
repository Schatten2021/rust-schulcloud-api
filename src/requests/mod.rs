use serde::Serialize;
use serde::de::{DeserializeOwned, IntoDeserializer};
use crate::state::State;
use crate::errors::{Errors, Result};
use crate::request_types::APIResponse;

pub mod login;
pub mod about;
pub mod chats;
#[cfg(feature = "blocking")]
pub mod blocking;

pub async fn post_request<T: DeserializeOwned>(state: &State, path: impl ToString, data: impl Serialize) -> Result<T> {
    let url = state.build_url(path);
    let response = reqwest::Client::new()
        .post(url)
        .form(&data)
        .header("Accept", "application/json")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0")
        .send().await?
        .json::<APIResponse>().await.map_err(|e| Errors::NotJsonError(e))?;
    if response.status.value != "OK" {
        return Err(Errors::APIError(response.status.value, response.status.short_message, response.status.message))
    }
    T::deserialize(response.payload.clone().into_deserializer())
        .map_err(|e| {
            println!("{}", response.payload);
            Errors::JsonDeserializeError(e)
        })
}