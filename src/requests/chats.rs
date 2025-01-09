use std::fmt::{write, Display, Formatter};
use std::path::PathBuf;
use base64::Engine;
use openssl::bn::BigNum;
use openssl::rsa::Padding;
use openssl::sha::sha256;
use openssl::symm::{decrypt, Cipher};
use crate::errors::Errors;
use crate::types::chats::channels::Channel;
use crate::Result;
use crate::state::{EncryptionState, State};
use crate::request_types::{chats, AuthOnlyRequest};
use crate::request_types::about::PublicSigningKey;
use crate::requests::about::get_other_user_info;
use crate::requests::post_request;
use crate::types::chats::conversations::Conversation;
use crate::types::chats::messages::{File, Message, PossibleSender};
use crate::types::user::others::User;

pub async fn get_channels(state: &State, company_id: impl ToString) -> Result<Vec<Channel>> {
    Ok(post_request::<chats::ChannelsResponse>(state,
                                               "/channels/subscripted",
                                               chats::ChannelRequest::new(state, company_id.to_string())?)
        .await?.channels)
}
pub async fn get_conversations(state: &State, limit: usize, offset: usize, archive: usize, sorting: Vec<String>) -> Result<Vec<Conversation>> {
    Ok(post_request::<chats::ConversationResponse>(state,
                                               "/message/conversations",
                                               chats::ConversationsRequest::new(state, limit, offset, archive, sorting)?)
        .await?.conversations)
}

pub async fn get_messages(state: &State, id: String, chat_type: ChatType, limit: usize, offset: usize, key: Option<Vec<u8>>) -> Result<Vec<Message>> {
    let mut messages = post_request::<chats::MessageResponse>(state,
                                                          "/message/content",
                                                          chats::MessageRequest::new(state, id, chat_type.to_string(), limit, offset)?)
        .await?.messages;
    let Some(key) = key else {
        return Ok(messages);
    };
    for message in &mut messages {
        if message.encrypted != Some(true) || message.text.is_none() {
            continue;
        }
        let encrypted = hex::decode(message.text.clone().unwrap()).map_err(|e| Errors::HexError(e))?;
        let iv_ = match &message.iv {
            Some(iv) => Some(hex::decode(iv).map_err(|e| Errors::HexError(e))?),
            None => None,
        };
        let iv = match &iv_ {
            Some(iv) => Some(&**iv),
            None => None
        };
        let decrypted = decrypt(Cipher::aes_256_cbc(), &*key, iv, &*encrypted)
            .map_err(|e| Errors::EncryptionError(e))?;
        let text = String::from_utf8(decrypted).map_err(|e| Errors::StringDecodeError(e))?;
        message.text = Some(text);
    }
    Ok(messages)
}
pub async fn download_file(state: &State, key: Option<Vec<u8>>, file: File) -> Result<Vec<u8>> {
    let raw_data = reqwest::Client::new()
        .post(state.build_url("/file/download"))
        .query(&chats::FileDownloadQuery { id: file.id })
        .form(&AuthOnlyRequest::new(state)?)
        .send().await
        .map_err(|e| Errors::RequestError(e))?
        .bytes().await
        .map_err(|e| Errors::RequestError(e))?
        .to_vec();
    if !file.encrypted || key.is_none() {
        return Ok(raw_data);
    }
    let iv_ = match file.e2e_iv {
        Some(iv) => Some(hex::decode(iv).map_err(|e| Errors::HexError(e))?),
        None => None
    };
    let iv = match &iv_ {
        Some(iv) => Some(&**iv),
        None => None,
    };
    decrypt(Cipher::aes_256_cbc(), key.unwrap().as_ref(), iv, &*raw_data)
        .map_err(|e| Errors::EncryptionError(e))
}
pub async fn verify_signature(state: &State, message: &Message) -> Result<bool> {
    if message.verification.is_none() {
        return Ok(true);
    }
    let sender_id = match &message.sender {
        PossibleSender::MessageSender(s) => s.id.clone().unwrap(),
        PossibleSender::String(s) => return Err(Errors::OtherErrors("Can't verify hash of unknown sender.".to_string()))
    };
    let user_info: User = get_other_user_info(state, sender_id).await?;
    let public_signing_key_obj = serde_json::from_str::<PublicSigningKey>(&*user_info.public_signing_key)
        .map_err(|e| Errors::JsonDeserializeError(e))?;
    let engine = base64::engine::general_purpose::URL_SAFE;
    let n = BigNum::from_slice(&*engine.decode(public_signing_key_obj.n)
        .map_err(|e| Errors::Base64Error(e))?)
        .map_err(|e| Errors::EncryptionError(e))?;
    let e = BigNum::from_slice(&*engine.decode(public_signing_key_obj.e)
        .map_err(|e| Errors::Base64Error(e))?)
        .map_err(|e| Errors::EncryptionError(e))?;
    let key = openssl::rsa::Rsa::from_public_components(n, e)
        .map_err(|e| Errors::EncryptionError(e))?;
    let data = message.text.clone().unwrap();
    let mut buff = vec![0; 256];
    let _written = key.public_encrypt(&sha256(data.as_ref()), &mut *buff, Padding::NONE)
        .map_err(|e| Errors::EncryptionError(e))?;
    let target = hex::decode(message.verification.clone().unwrap())
        .map_err(|e| Errors::HexError(e))?;
    Ok(target != buff)
}
pub enum ChatType {
    Channel,
    Conversation,
}
impl Display for ChatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ChatType::Channel => "channel",
            ChatType::Conversation => "conversation",
        })
    }
}