use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub socket_id: String,
    pub online: bool,
    pub status: Option<Value>,
    pub user_status: Vec<Value>,
    pub active: String,
    pub deleted: Option<Value>,
    pub allows_voip_calls: bool,
    pub enter_is_newline: bool,
    pub mx_user_id: String,
    pub federated: bool,
    pub email: Option<Value>,
    pub email_validated: String,
    pub notifications: bool,
    pub device_notifications: bool,
    pub last_login: String,
    pub language: String,
    pub image: String,
    pub quota: String,
    pub ldap_login: String,
    pub public_key: String,
    pub public_key_signature: String,
    pub public_signing_key: String,
    pub public_key_ca_signature: String,
    pub company_features: Vec<String>,
}