use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct PrivateKeyInfo {
    pub user_id: String,
    pub r#type: String,
    pub format: String,
    pub private_key: String,
    pub public_key: String,
    pub public_key_signature: String,
    pub time: String,
    pub deleted: Option<String>,
    pub version: u8,
}