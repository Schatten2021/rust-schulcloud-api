use serde::Deserialize;
use serde_json::Value;
use crate::networking::data::user::{UserRole, UserSettings};

#[derive(Deserialize, Clone, Debug)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub quota: String,
    pub max_users: Value,
    pub created: String,
    pub deleted: Option<String>,
    pub online_payment: String,
    pub freemium: String,
    pub logo: String,
    pub logo_url: String,
    pub users: CompanyUsers,
    pub features: Vec<String>,
    pub marketplace_modules: Vec<String>,
    pub provider: String,
    pub protected: bool,
    pub roles: Vec<UserRole>,
    pub permissions: Option<Vec<String>>,
    pub settings: UserSettings,
    pub domains: Vec<Value>,
    pub domain: String,
    pub time_joined: String,
    pub membership_expiry: Option<String>,
    pub deactivated: Option<String>,
    pub maps: Vec<Value>,
    pub unread_messages: u16
}
#[derive(Deserialize, Clone, Debug)]
pub struct CompanyUsers {
    pub created: u32,
    pub active: u32,
}