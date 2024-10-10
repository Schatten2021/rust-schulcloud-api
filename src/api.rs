use crate::errors::Errors;
use crate::networking::connection::{Config, Connection, MessageSource};
use crate::networking::data::channel_info::Channel;
use crate::networking::data::companies::Company;
use crate::networking::data::conversation_info::Conversation;
use crate::networking::data::messages::{File, Message};
use crate::networking::data::user::UserInfo;
use base64::Engine;
use openssl::error::ErrorStack;
use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};
use openssl::symm::{decrypt, Cipher};
use serde_json::Value;
use std::collections::HashMap;

const BASE64ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD, base64::engine::GeneralPurposeConfig::new());
const MESSAGES_PER_REQUEST: u32 = 64;
const CONVERSATIONS_PER_REQUEST: u32 = 64;

pub struct Api {
    conn: Connection,
    user_info: Option<UserInfo>,
    logged_in: bool,
    private_key: Option<Rsa<Private>>,
    channel_infos: Vec<Channel>,
    conversation_infos: Vec<Conversation>,
    key_store: HashMap<ChatID, Vec<u8>>,
    message_store: HashMap<ChatID, Vec<Message>>,
    companies: Vec<Company>,
    passphrase: Option<String>,
}

impl Api {
    async fn get_private_key(&mut self, passphrase: String) -> Result<Rsa<Private>, Errors> {
        if let Some(key) = self.private_key.clone() {
            return Ok(key);
        }
        let key_info = self.conn.get_private_key().await?;
        let private_key_json: Result<Value, serde_json::Error> = serde_json::from_str(&*key_info.private_key);
        if let Err(e) = private_key_json {
            return Err(Errors::JsonDeserializeError(e));
        }
        let private_key_json = private_key_json.unwrap();
        let encrypted_pem_key = private_key_json.get("private");
        if encrypted_pem_key.is_none() {
            return Err(Errors::OtherErrors("Couldn't load private key from field \"private\"".to_string()));
        }
        let encrypted_pem_key = encrypted_pem_key.unwrap();
        let encrypted_pem_key: Option<&str> = encrypted_pem_key.as_str();
        if encrypted_pem_key.is_none() {
            return Err(Errors::OtherErrors("Private key is not in PEM format".to_string()));
        }
        let encrypted_pem_key = encrypted_pem_key.unwrap();
        let key = Rsa::private_key_from_pem_passphrase(encrypted_pem_key.as_bytes(), passphrase.as_bytes());
        if let Err(e) = key {
            return Err(Errors::EncryptionError(e))
        }
        let key = key.unwrap();
        self.private_key = Some(key.clone());
        Ok(key)
    }
    async fn decrypt_key(&mut self, key: String) -> Result<Vec<u8>, Errors> {
        if self.passphrase.is_none() {
            return Err(Errors::ValueError("Passphrase is None! Passphrase needs to be set before accessing anything encrypted".to_string()))
        }
        let encrypted_data = BASE64ENGINE.decode(key);
        if let Err(e) = encrypted_data {
            return Err(Errors::Base64Error(e));
        }
        let encrypted_data = encrypted_data.unwrap();
        let mut result_buf: Vec<u8> = Vec::new();
        result_buf.resize(encrypted_data.clone().len(), 0);
        let private_key = self.get_private_key(self.passphrase.clone().unwrap()).await?;
        let decrypted_key_length = private_key.private_decrypt(&*encrypted_data, &mut result_buf, Padding::PKCS1_OAEP);
        if let Err(e) = decrypted_key_length {
            return Err(Errors::EncryptionError(e));
        }
        let decrypted_key_length = decrypted_key_length.unwrap();
        result_buf.resize(decrypted_key_length, 0);
        Ok(result_buf)
    }
    async fn decrypt(&mut self, chat: ChatID, data: Vec<u8>, iv: Option<String>) -> Result<Vec<u8>, Errors> {
        if !self.key_store.contains_key(&chat) {
            if chat.r#type == ChatType::Channel {
                let channel = self.channel_infos.iter()
                    .find(|channel| channel.id == chat.id);
                if channel.is_none() {
                    return Err(Errors::ValueError("Invalid channel id".to_string()));
                }
                let channel = channel.unwrap();
                if !channel.encrypted.clone() {
                    return Ok(data);
                }
                if channel.key.is_none() {
                    return Err(Errors::ValueError("Key is none for channel \"".to_string() + &*channel.name + "\""))
                }
                let decrypted_key = self.decrypt_key(channel.key.clone().unwrap()).await?;
                self.key_store.insert(chat.clone(), decrypted_key);
            } else if chat.r#type == ChatType::Conversation {
                let conversation = self.conversation_infos
                    .iter().find(|conversation| conversation.id == chat.id);
                if conversation.is_none() {
                    return Err(Errors::ValueError("Invalid conversation id".to_string()));
                }
                let conversation = conversation.unwrap();
                if !conversation.encrypted.clone() {
                    return Ok(data);
                }
                let decrypted_key = self.decrypt_key(conversation.key.clone().unwrap()).await?;
                self.key_store.insert(chat.clone(), decrypted_key);
            } else {
                return Err(Errors::ValueError(format!("Unknown chat type: {:?}", chat.r#type)))
            }
        }

        //actual decryption
        let key = self.key_store.get(&chat).expect("Key is none although key is set in keystore.");
        let decrypted: Result<Vec<u8>, ErrorStack> = match iv {
            None => decrypt(Cipher::aes_256_cbc(), key, None, &*data),
            Some(iv) => match hex::decode(iv) {
                Err(e) => return Err(Errors::HexError(e)),
                Ok(iv) => decrypt(Cipher::aes_256_cbc(), key, Some(&*iv), &*data)
            }
        };
        match decrypted {
            Err(e) => Err(Errors::EncryptionError(e)),
            Ok(v) => Ok(v)
        }
    }
    async fn decrypt_message(&mut self, chat: ChatID, message: Message) -> Result<Option<String>, Errors>{
        if message.encrypted.is_none() || !message.encrypted.unwrap() || message.text.is_none() || message.text.clone().unwrap().len() == 0{
            return Ok(message.text);
        }
        let data = match hex::decode(message.text.unwrap()) {
            Err(e) => return Err(Errors::HexError(e)),
            Ok(d) => d
        };
        let decrypted = self.decrypt(chat, data, message.iv).await?;
        match String::from_utf8(decrypted) {
            Err(e) => Err(Errors::StringDecodeError(e)),
            Ok(s) => Ok(Some(s))
        }
    }
}
impl Api {
    pub fn new() -> Self {
        Api {
            conn: Connection::new(Config::new("https://api.stashcat.com/".to_string())),
            user_info: None,
            logged_in: false,
            private_key: None,
            channel_infos: Vec::new(),
            conversation_infos: Vec::new(),
            key_store: HashMap::new(),
            message_store: HashMap::new(),
            companies: Vec::new(),
            passphrase: None,
        }
    }
    pub async fn new_logged_in(device_id: String, client_key: String) -> Result<Self, Errors> {
        let mut api = Api {
            conn: Connection::new(Config{
                base_url: "https://api.stashcat.com/".to_string(),
                device_id,
                client_key: Some(client_key),
            }),
            user_info: None,
            logged_in: true,
            private_key: None,
            channel_infos: Vec::new(),
            conversation_infos: Vec::new(),
            key_store: HashMap::new(),
            message_store: HashMap::new(),
            companies: Vec::new(),
            passphrase: None,
        };
        api.update_user_info().await?;
        api.post_login().await?;
        Ok(api)
    }
    pub async fn email_password_login(&mut self, email: String, password: String, app_name: String) -> Result<(), Errors> {
        let dat = self.conn.email_password_login(email, password, app_name).await?;
        self.user_info = Some(dat);
        self.post_login().await?;
        Ok(())
    }
    pub async fn update_user_info(&mut self) -> Result<(), Errors>{
        self.user_info = Some(self.conn.get_user_info().await?);
        Ok(())
    }
    async fn post_login(&mut self) -> Result<(), Errors>{
        self.logged_in = true;
        self.companies = self.conn.get_companies().await?;
        for company in self.companies.clone() {
            self.channel_infos.extend(self.conn.get_channels(company.id).await?);
        };
        let mut requested_conversations: u64= 0;
        while self.conversation_infos.clone().len() == requested_conversations as usize {
            self.conversation_infos.extend(self.conn.get_conversations(CONVERSATIONS_PER_REQUEST, requested_conversations, "0".to_string(), "[\"favorite_desc\",\"last_action_desc\"]".to_string()).await?);
            requested_conversations += u64::from(CONVERSATIONS_PER_REQUEST);
        };
        Ok(())
    }
    pub fn set_passphrase(&mut self, passphrase: String) {self.passphrase = Some(passphrase)}

    pub fn get_channel_ids(&self) -> Vec<String> {
        self.channel_infos.iter().map(|channel| channel.id.clone()).collect()
    }
    pub fn get_conversation_ids(&self) -> Vec<String> {
        self.conversation_infos
            .iter()
            .map(|conversation| conversation.id.clone())
            .collect()
    }
    pub fn get_channels(&self) -> Vec<Channel> {
        self.channel_infos.clone()
    }
    pub fn get_conversations(&self) -> Vec<Conversation> {
        self.conversation_infos.clone()
    }
    pub async fn get_messages(&mut self, chat: ChatID) -> Result<Vec<Message>, Errors>{
        if self.message_store.contains_key(&chat) {
            return Ok(self.message_store.get(&chat).expect("Message Store contains Chat but has no data").clone());
        }
        let mut requested_message_count: u64 = 0;
        let mut messages: Vec<Message> = Vec::new();
        while messages.len() == requested_message_count as usize{
            let mut new_messages: Vec<Message> = self.conn.get_messages(chat.id.clone(),
                                                                        match chat.r#type {
                                                                            ChatType::Channel => MessageSource::Channel,
                                                                            ChatType::Conversation => MessageSource::Conversation
                                                                        },
                                                                        MESSAGES_PER_REQUEST,
                                                                        requested_message_count,
            ).await?;
            requested_message_count += MESSAGES_PER_REQUEST as u64;
            new_messages.extend(messages);
            messages = new_messages;
        }
        self.message_store.insert(chat, messages.clone());
        Ok(messages)
    }
    pub async fn get_decrypted_messages(&mut self, source: ChatID) -> Result<Vec<Option<String>>, Errors> {
        let messages = self.get_messages(source.clone()).await?;
        let mut decrypted: Vec<Option<String>> = Vec::new();
        for message in messages {
            decrypted.push(self.decrypt_message(source.clone(), message.clone()).await?);
        }
        Ok(decrypted)
    }
    pub async fn download_file(&mut self, chat: ChatID, file: File) -> Result<Vec<u8>, Errors> {
        let encrypted = self.conn.download_file(file.id).await?;
        if !file.encrypted {
            return Ok(encrypted);
        }
        Ok(self.decrypt(chat, encrypted, file.e2e_iv).await?)
    }
    pub fn is_logged_in(&self) -> bool {self.logged_in}
    pub fn has_passphrase_set(&self) -> bool {!self.passphrase.is_none()}
    pub fn get_chat_name(&self, chat: ChatID) -> Result<String, Errors> {
        match chat.r#type {
            ChatType::Conversation => {
                let conversation = self.conversation_infos.iter().find(|conversation| conversation.id == chat.id);
                match conversation {
                    None => Err(Errors::ValueError(format!("No conversation found with id \"{}\"", chat.id))),
                    Some(conversation) => match conversation.name.clone() {
                        Some(s) => Ok(s),
                        None => Err(Errors::ValueError(format!("Conversation with id {} has no name", chat.id)))
                    }
                }
            }
            ChatType::Channel => {
                let channel = self.channel_infos.iter().find(|channel| channel.id == chat.id);
                match channel {
                    None => Err(Errors::ValueError(format!("No channel found with id \"{}\"", chat.id))),
                    Some(channel) => Ok(channel.name.clone())
                }
            }
        }
    }
}
#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct  ChatID {
    r#type: ChatType,
    id: String,
}
impl ChatID {
    pub fn new(r#type: ChatType, id: String) -> Self {
        ChatID {
            r#type,
            id,
        }
    }
    pub fn from_channel(channel: Channel) -> Self {
        Self::new(ChatType::Channel, channel.id)
    }
    pub fn from_conversation(conversation: Conversation) -> Self {
        Self::new(ChatType::Conversation, conversation.id)
    }
}
#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub enum ChatType {
    Conversation,
    Channel,
}