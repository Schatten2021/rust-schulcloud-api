pub mod connection;
pub mod data;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct APIResponseStatus {
    value: String,
    short_message: String,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    status: APIResponseStatus,
    payload: Value,
    signature: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct DetailedPersonInfo {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image: Option<String>,
    pub active: Option<String>,
    pub deleted: Option<String>,
    pub allows_voip_calls: bool,
    pub mx_user_id: Option<String>,
    pub federated: bool,
    pub online: bool,
    pub public_key: Option<String>,
    pub public_key_signature: Option<String>,
    pub public_signing_key: Option<String>,
    pub public_key_ca_signature: Option<String>,
    pub language: Option<String>,
}