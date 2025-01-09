use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct EmailPasswordLogin {
    pub email: String,
    pub password: String,
    pub device_id: String,
    pub app_name: String,
    pub encrypted: String,
    pub callable: String,
    pub key_transfer_support: String,
}
impl EmailPasswordLogin {
    pub fn new(email: String, password: String, device_id: String, app_name: String, encrypted: bool, callable: bool, key_transfer_support: bool) -> Self {
        Self {
            email, password, device_id, app_name,
            encrypted: encrypted.to_string(),
            callable: callable.to_string(),
            key_transfer_support: key_transfer_support.to_string(),
        }
    }
}
#[derive(Deserialize)]
pub(crate) struct LoginSuccessResponse {
    pub(crate) client_key: String,
    pub(crate) userinfo: Value,
}