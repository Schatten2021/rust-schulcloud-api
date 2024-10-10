use crate::errors::Errors;
use crate::networking::data::channel_info::Channel;
use crate::networking::data::conversation_info::Conversation;
use crate::networking::data::messages::Message;
use crate::networking::data::private_key::PrivateKeyInfo;
use crate::networking::data::user::UserInfo;
use crate::networking::APIResponse;
use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::Client;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use crate::networking::data::companies::Company;

pub(crate) struct Config{
    pub(crate) base_url: String,
    pub(crate) device_id: String,
    pub(crate) client_key: Option<String>
}
impl Config {
    pub(crate) fn new(base_url: String) -> Config {
        Config {
            base_url,
            device_id: rand::thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect(),
            client_key: None
        }
    }
}
pub(crate) struct Connection {
    base_url: String,
    device_id: String,
    client_key: Option<String>,
    pub(crate) client: Client,
}

impl Connection {
    pub(crate) fn new(config: Config) -> Self {
        Connection {
            base_url: config.base_url,
            device_id: config.device_id,
            client_key: config.client_key,
            client: Client::new(),
        }
    }
    pub(crate) fn get_device_id(&self) -> String {self.device_id.clone() }
    pub(crate) fn get_client_key(&self) -> String {self.client_key.clone().unwrap()}
    pub(crate) async fn post_request(&self, extended_url: &str, data: impl Serialize) -> Result<APIResponse, Errors> {
        let url = self.base_url.clone() + extended_url;
        let response = self.client
            .post(url)
            .form(&data)
            .header("Accept", "application/json")
            .send().await;
        if let Err(e) = response {
            return Err(Errors::RequestError(e))
        }
        let response = response.unwrap();
        let json = response.json::<APIResponse>().await;
        if let Err(e) = json {
            return Err(Errors::NotJsonError(e))
        }
        let json = json.unwrap();
        if json.status.value != "OK" {
            return Err(Errors::APIError(json.status.short_message, json.status.message));
        }
        Ok(json)
    }
    pub(crate) async fn email_password_login(&mut self, email: String, password: String, app_name: String) -> Result<UserInfo, Errors> {
        let data: EmailPasswordLoginRequestBody = EmailPasswordLoginRequestBody {
            email,
            password,
            device_id: self.get_device_id(),
            app_name,
            encrypted: "false".to_string(),
            callable: "false".to_string(),
            key_transfer_support: "false".to_string(),
        };
        let response = self.post_request("/auth/login", &data).await?;
        let result = LoginSuccessResponse::deserialize(response.payload.into_deserializer());
        if let Err(e) = result {
            return Err(Errors::JsonDeserializeError(e));
        }
        let result = result.unwrap();
        self.client_key = Some(result.client_key.clone());
        Ok(result.userinfo)
    }
    pub(crate) async fn get_conversations(&self, limit: u32, offset: u64, archive: String, sorting: String) -> Result<Vec<Conversation>, Errors> {
        let request_body = GetConversationRequestBody {
            client_key: self.get_client_key(),
            device_id: self.get_device_id(),
            limit: limit.to_string(),
            offset: offset.to_string(),
            archive,
            sorting,
        };
        let response = self.post_request("/message/conversations", request_body).await?;
        let json = GetConversationRequestResult::deserialize(response.payload.into_deserializer());
        if let Err(e) = json {
            return Err(Errors::JsonDeserializeError(e));
        }
        let json = json.unwrap();
        Ok(json.conversations)
    }
    pub(crate) async fn get_channels(&self, company: String) -> Result<Vec<Channel>, Errors> {
        let request_body = GetChannelsRequestBody {
            client_key: self.get_client_key(),
            device_id: self.get_device_id(),
            company,
        };
        let response = self.post_request("/channels/subscripted", request_body).await?;
        let json = GetChannelsRequestResult::deserialize(response.payload.clone().into_deserializer());
        if let Err(e) = json {
            println!("{}", response.payload);
            return Err(Errors::JsonDeserializeError(e));
        }
        let json = json.unwrap();
        Ok(json.channels)
    }
    pub(crate) async fn get_private_key(&self) -> Result<PrivateKeyInfo, Errors> {
        let request_body = SimpleAuthRequestBody {
            client_key: self.get_client_key(),
            device_id: self.get_device_id(),
        };
        let response = self.post_request("security/get_private_key", request_body).await?;
        let json = GetPrivateKeyResponse::deserialize(response.payload.into_deserializer());
        if let Err(e) = json {
            return Err(Errors::JsonDeserializeError(e));
        }
        let json = json.unwrap();
        Ok(json.keys)
    }
    pub(crate) async fn get_messages(&self, id: String, source: MessageSource, limit: u32, offset: u64) -> Result<Vec<Message>, Errors> {
        let source = match source {MessageSource::Conversation => "conversation", MessageSource::Channel => "channel"};
        let request_body = GetMessagesRequestBody {
            client_key: self.get_client_key(),
            device_id: self.get_device_id(),
            channel_id: id.clone(),
            conversation_id: id,
            source: source.to_string(),
            limit: limit.to_string(),
            offset: offset.to_string(),
        };
        let response = self.post_request("/message/content", request_body).await?;
        let json = GetMessagesRequestResult::deserialize(response.payload.into_deserializer());
        if let Err(e) = json {
            return Err(Errors::JsonDeserializeError(e));
        }
        let json = json.unwrap();
        Ok(json.messages)
    }
    pub(crate) async fn download_file(&self, id: String) -> Result<Vec<u8>, Errors> {
        let request_body = SimpleAuthRequestBody {
            client_key: self.get_client_key(),
            device_id: self.get_device_id(),
        };
        let request_query_params = FileDownloadRequestQueryParams {id};
        let url = self.base_url.clone() + "/file/download";
        let response = self.client
            .post(url)
            .query(&request_query_params)
            .form(&request_body)
            .send().await;
        if let Err(e) = response {
            return Err(Errors::RequestError(e));
        }
        let response = response.unwrap();
        let raw = response.bytes().await;
        if let Err(e) = raw {
            return Err(Errors::RequestError(e))
        }
        let raw = raw.unwrap();
        Ok(raw.to_vec())
    }
    pub(crate) async fn get_companies(&self) -> Result<Vec<Company>, Errors> {
        let request_body = GetCompaniesRequestBody {
            device_id: self.get_device_id(),
            client_key: self.get_client_key(),
            no_cache: "true".to_string(),
        };
        let response = self.post_request("/company/member", request_body).await?;
        let json = GetCompaniesRequestResult::deserialize(response.payload.into_deserializer());
        if let Err(e) = json {
            return Err(Errors::JsonDeserializeError(e));
        }
        let json = json.unwrap();
        Ok(json.companies)
    }
    pub(crate) async fn get_user_info(&self) -> Result<UserInfo, Errors> {
        let request_header = GetUserInfoRequestBody {
            client_key: self.get_client_key(),
            device_id: self.get_device_id(),
            withkey: "false".to_string(),
        };
        let response = self.post_request("/users/me", request_header).await?;
        let json = GetUserInfoRequestResult::deserialize(response.payload.into_deserializer());
        if let Err(e) = json {
            return Err(Errors::JsonDeserializeError(e));
        }
        Ok(json.unwrap().user)
    }
}
//request data
#[derive(Serialize)]
struct SimpleAuthRequestBody {
    client_key: String,
    device_id: String,
}

#[derive(Serialize)]
struct EmailPasswordLoginRequestBody {
    email: String,
    password: String,
    device_id: String,
    app_name: String,
    encrypted: String,
    callable: String,
    key_transfer_support: String,
}
#[derive(Deserialize)]
struct LoginSuccessResponse {
    client_key: String,
    userinfo: UserInfo,
}

#[derive(Serialize)]
struct GetConversationRequestBody {
    client_key: String,
    device_id: String,
    limit: String,
    offset: String,
    archive: String,
    sorting: String,
}
#[derive(Deserialize)]
struct GetConversationRequestResult {
    conversations: Vec<Conversation>,
}

#[derive(Serialize)]
struct GetChannelsRequestBody {
    client_key: String,
    device_id: String,
    company: String,
}
#[derive(Deserialize)]
struct GetChannelsRequestResult {
    channels: Vec<Channel>,
}

#[derive(Deserialize)]
struct GetPrivateKeyResponse {
    keys: PrivateKeyInfo,
}

#[derive(Serialize)]
struct GetMessagesRequestBody {
    client_key: String,
    device_id: String,
    conversation_id: String,
    channel_id: String,
    source: String,
    limit: String,
    offset: String,
}
pub(crate) enum MessageSource {
    Conversation,
    Channel,
}
#[derive(Deserialize)]
struct GetMessagesRequestResult {
    messages: Vec<Message>
}

#[derive(Serialize)]
struct FileDownloadRequestQueryParams {
    id: String,
}

#[derive(Serialize)]
struct GetCompaniesRequestBody {
    client_key: String,
    device_id: String,
    no_cache: String,
}
#[derive(Deserialize)]
struct GetCompaniesRequestResult {
    companies: Vec<Company>,
}

#[derive(Serialize)]
struct GetUserInfoRequestBody {
    client_key: String,
    device_id: String,
    withkey: String,
}
#[derive(Deserialize)]
struct GetUserInfoRequestResult {
    user: UserInfo,
}