use crate::errors::Errors;
use crate::request_types::about::RSAPublicKey;
use crate::request_types::{chats, AuthOnlyRequest};
use crate::requests::blocking::about::get_other_user_info;
use crate::requests::blocking::post_request;
use crate::state::State;
use crate::types::chats::channels::Channel;
use crate::types::chats::conversations::Conversation;
use crate::types::chats::messages::{File, Message, PossibleSender};
use crate::types::user::others::User;
use crate::Result;
use base64::Engine;
use openssl::rsa::Padding;
use openssl::symm::{decrypt, Cipher};
use std::fmt::{Display, Formatter};

pub fn get_channels(state: &State, company_id: impl ToString) -> Result<Vec<Channel>> {
    Ok(post_request::<chats::ChannelsResponse>(state,
                                               "/channels/subscripted",
                                               chats::ChannelRequest::new(state, company_id.to_string())?)
        ?.channels)
}
pub fn get_conversations(state: &State, limit: usize, offset: usize, archive: usize, sorting: Vec<String>) -> Result<Vec<Conversation>> {
    Ok(post_request::<chats::ConversationResponse>(state,
                                               "/message/conversations",
                                               chats::ConversationsRequest::new(state, limit, offset, archive, sorting)?)?
        .conversations)
}

pub fn get_messages(state: &State, id: String, chat_type: ChatType, limit: usize, offset: usize, key: Option<Vec<u8>>) -> Result<Vec<Message>> {
    let mut messages = post_request::<chats::MessageResponse>(state,
                                                          "/message/content",
                                                          chats::MessageRequest::new(state, id, chat_type.to_string(), limit, offset)?)?
        .messages;
    let _ = messages.iter_mut().map(|m| m.original_text = m.text.clone()).collect::<Vec<()>>();
    let Some(key) = key else {
        return Ok(messages);
    };
    for message in &mut messages {
        if message.encrypted != Some(true) || message.text.is_none() || message.text == Some("".to_string()) {
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
pub fn download_file(state: &State, key: Option<Vec<u8>>, file: File) -> Result<Vec<u8>> {
    let raw_data = reqwest::blocking::Client::new()
        .post(state.build_url("/file/download"))
        .query(&chats::FileDownloadQuery { id: file.id })
        .form(&AuthOnlyRequest::new(state)?)
        .send()?
        .bytes()?
        .to_vec();
    if !file.encrypted || key.is_none() {
        return Ok(raw_data);
    }
    let iv_ = match file.e2e_iv {
        Some(iv) => Some(hex::decode(iv)?),
        None => None
    };
    let iv = match &iv_ {
        Some(iv) => Some(&**iv),
        None => None,
    };
    decrypt(Cipher::aes_256_cbc(), key.unwrap().as_ref(), iv, &*raw_data)
        .map_err(|e| Errors::EncryptionError(e))
}
#[cfg(feature = "experimental")]
pub fn verify_signature(state: &State, message: &Message) -> Result<bool> {
    if message.verification.is_none() {
        return Ok(true);
    }
    let sender_id = match &message.sender {
        PossibleSender::MessageSender(s) => s.id.clone().unwrap(),
        PossibleSender::String(_s) => return Err(Errors::OtherErrors("Can't verify hash of unknown sender.".to_string()))
    };
    let target = hex::decode(message.verification.clone().unwrap())?;
    let user_info: User = get_other_user_info(state, sender_id)?;
    let rsa_key = RSAPublicKey::from_str(&*user_info.public_signing_key)?.to_key()?;
    let key = openssl::pkey::PKey::from_rsa(rsa_key.clone())?;
    let data = message.text.clone().unwrap().into_bytes();
    if message.encrypted == Some(true) {
        hex::decode(message.original_text.clone().unwrap())?
    } else {
        message.original_text.clone().unwrap().into_bytes()
    };
    let target_hash = base64::engine::general_purpose::STANDARD.decode(message.hash.clone().unwrap())?;
    let generated_hash = openssl::hash::hash(openssl::hash::MessageDigest::sha256(), &*data)?.to_vec();
    println!("{}", generated_hash == target_hash);
    let mut buff = vec![0; rsa_key.size() as usize];
    let buff_len = rsa_key.public_decrypt(&*generated_hash, &mut buff, Padding::NONE)?;
    buff.truncate(buff_len);
    println!("{}", buff == target);
    println!("{:?}", buff);
    println!("{buff_len}");

    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &key)?;
    verifier.update(&*data)?;
    let res = verifier.verify(&*target).map_err(Errors::from);
    res
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