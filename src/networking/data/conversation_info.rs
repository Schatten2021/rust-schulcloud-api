use serde::Deserialize;
use serde_json::Value;
use crate::networking::DetailedPersonInfo;

#[derive(Deserialize, Clone, Debug)]
pub struct Conversation {
    pub id: String,
    pub name: Option<String>,
    pub created: Option<String>,
    pub last_action: Option<String>,
    pub last_activity: Option<String>,
    pub encrypted: bool,
    pub unique_identifier: Value,
    pub unread_messages: u32,
    pub key: Option<String>,
    pub key_requested: Value,
    pub key_signature: Value,
    pub key_sender: Value,
    pub signature_expiry: Value,
    pub archive: Value,
    pub favorite: bool,
    pub deleted: Option<String>,
    pub muted: Value,
    pub user_count: u8,
    pub members: Vec<DetailedPersonInfo>,
    pub num_members_without_keys: u8,
    pub members_without_keys: Vec<Value>,
    pub callable: Vec<DetailedPersonInfo>,
}