use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::DetailedPersonInfo;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Message {
    pub id: u64,
    pub text: Option<String>,
    pub conversation_id: Option<u64>,
    pub channel_id: Option<u64>,
    pub thread_id: Option<u64>,
    pub hash: Option<String>,
    pub verification: Option<String>,
    pub broadcast: Value,
    pub alarm: bool,
    pub confirmation_required: bool,
    pub confirmations: Vec<Value>,
    pub time: Option<String>,
    pub micro_time: Option<String>,
    pub sender: PossibleSender,
    pub device: Option<String>,
    pub device_id: Option<String>,
    pub deleted: Option<String>,
    pub kind: Option<String>,
    pub r#type: Option<String>,
    pub location: Option<MessageLocation>,
    pub is_forwarded: Option<bool>,
    pub metainfo: Option<String>,
    pub messagePayload: Option<Value>,
    pub has_file_attached: Option<bool>,
    pub reciever: Option<Vec<Value>>,
    pub files: Option<Vec<File>>,
    pub likes: Option<u64>,
    pub liked: Option<bool>,
    pub flagged: Option<bool>,
    pub tags: Option<Vec<Value>>,
    pub links: Option<Vec<Value>>,
    pub seen: Option<Vec<MessageSeen>>,
    pub seen_by_others: Option<bool>,
    pub unread: Option<bool>,
    pub encrypted: Option<bool>,
    pub iv: Option<String>,
    pub reply_to: Option<PossibleReply>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum PossibleReply {
    MessageReplyTo(MessageReplyTo),
    Integer(u64),
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum PossibleSender {
    MessageSender(DetailedPersonInfo),
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct MessageLocation {
    pub longitude: Option<Value>,
    pub latitude: Option<Value>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct MessageSeen {
    pub user_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct MessageReplyTo {
    pub message_id: u64,
    pub message_hash: Option<String>,
    pub message_verification: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct File {
    pub base_64: Option<String>,
    pub deleted: Option<String>,
    pub dimensions: FileDimensions,
    pub e2e_iv: Option<String>,
    pub encrypted: bool,
    pub ext: Option<String>,
    pub folder_type: Option<String>,
    pub id: String,
    pub last_download: Option<String>,
    pub md5: Option<String>,
    pub mime: Option<String>,
    pub modified: Option<String>,
    pub name: String,
    pub owner: DetailedPersonInfo,
    pub owner_id: Option<String>,
    pub permission: Option<String>,
    pub size: Option<String>,
    pub size_byte: Option<String>,
    pub size_string: Option<String>,
    pub status: Option<String>,
    pub times_downloaded: Option<String>,
    pub type_id: Option<String>,
    pub uploaded: Option<String>,
    pub virtual_folder: Option<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct FileDimensions {
    pub height: Option<String>,
    pub width: Option<String>,
}
