use serde::{Deserialize, Serialize};
use crate::state::State;
use crate::Result;
use crate::types::chats::channels::Channel;
use crate::types::chats::conversations::Conversation;
use crate::types::chats::messages::Message;

#[derive(Serialize)]
pub struct ConversationsRequest {
    pub client_key: String,
    pub device_id: String,
    pub limit: String,
    pub offset: String,
    pub archive: String,
    pub sorting: String,
}
impl ConversationsRequest {
    pub fn new(state: &State, limit: usize, offset: usize, archive: usize, sorting: Vec<String>) -> Result<Self> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            limit: limit.to_string(),
            offset: offset.to_string(),
            archive: archive.to_string(),
            sorting: format!("[{}]", sorting.join(","))
        })
    }
}
#[derive(Deserialize, Debug)]
pub struct ConversationResponse {
    pub conversations: Vec<Conversation>,
}

#[derive(Serialize)]
pub struct ChannelRequest {
    pub client_key: String,
    pub device_id: String,
    pub company: String,
}
impl ChannelRequest {
    pub fn new(state: &State, company_id: String) -> Result<Self> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            company: company_id,
        })
    }
}
#[derive(Deserialize, Debug)]
pub struct ChannelsResponse {
    pub channels: Vec<Channel>,
}

#[derive(Serialize, Debug)]
pub struct MessageRequest {
    pub client_key: String,
    pub device_id: String,
    pub channel_id: String,
    pub conversation_id: String,
    pub source: String,
    pub limit: String,
    pub offset: String,
}
impl MessageRequest {
    pub fn new(state: &State, id: String, source: String, limit: usize, offset: usize) -> Result<Self> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            channel_id: id.clone(),
            conversation_id: id,
            source,
            limit: limit.to_string(),
            offset: offset.to_string(),
        })
    }
}
#[derive(Deserialize, Debug)]
pub struct MessageResponse {
    pub messages: Vec<Message>
}

#[derive(Serialize)]
pub struct FileDownloadQuery {
    pub id: String,
}

#[derive(Serialize)]
pub struct SendMessageRequest {
    pub client_key: String,
    pub device_id: String,
    pub target: String,
    pub conversation_id: String,
    pub channel_id: String,
    pub text: String,
    pub files: String,
    pub url: String,
    pub encrypted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
    pub verification: String,
    pub r#type: String,
    pub is_forwarded: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metainfo: Option<String>,
}
impl SendMessageRequest {
    pub fn new(state: &State, target: String, chat_id: String, text: String, files: Vec<String>, url: Vec<Option<String>>, encrypted: bool, iv: Option<String>, verification: String, r#type: String, is_forwarded: bool, metainfo: Option<String>) -> Result<SendMessageRequest> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            target,
            conversation_id: chat_id.clone(),
            channel_id: chat_id,
            text,
            files: format!("[{}]", files.join(",")),
            url: format!("[{}]", vec!["null"; url.len()].join(",")), // TODO
            encrypted: encrypted.to_string(),
            iv,
            verification,
            r#type,
            is_forwarded: is_forwarded.to_string(),
            metainfo,
        })
    }
}