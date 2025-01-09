use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::state::State;
use crate::Result;

pub(crate) mod login;
pub(crate) mod about;
pub(crate) mod chats;

#[derive(Deserialize, Debug)]
pub(crate) struct APIResponseStatus {
    pub(crate) value: String,
    pub(crate) short_message: String,
    pub(crate) message: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct APIResponse {
    pub(crate) status: APIResponseStatus,
    pub(crate) payload: Value,
    pub(crate) signature: String,
}
#[derive(Serialize)]
pub struct AuthOnlyRequest {
    client_key: String,
    device_id: String,
}
impl AuthOnlyRequest {
    pub fn new(state: &State) -> Result<Self> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
        })
    }
}