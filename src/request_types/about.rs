use crate::state::State;
use crate::types::user::general::UserInfo;
use crate::types::user::others::User;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct UserInfoRequest {
    pub client_key: String,
    pub device_id: String,
    pub withkey: String,
}
impl UserInfoRequest {
    pub fn new(state: &State, with_key: bool) -> Result<Self> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            withkey: with_key.to_string(),
        })
    }
}
#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub user: UserInfo,
}

#[derive(Serialize)]
pub struct CompanyRequest {
    pub client_key: String,
    pub device_id: String,
    pub no_cache: bool,
}
impl CompanyRequest {
    pub fn new(state: &State) -> Result<Self> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            no_cache: true,
        })
    }
}
#[derive(Debug, Deserialize)]
pub struct CompanyResponse {
    pub companies: Vec<crate::types::user::companies::Company>
}

#[derive(Serialize)]
pub struct PrivateKeyRequest {
    pub client_key: String,
    pub device_id: String,
    pub format: String,
    pub r#type: String
}
impl PrivateKeyRequest {
    pub fn new(state: &State, format: impl ToString, r#type: impl ToString) -> Result<Self> {
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            format: format.to_string(),
            r#type: r#type.to_string(),
        })
    }
}
#[derive(Debug, Deserialize)]
pub struct PrivateKeyResponse {
    pub keys: PrivateKeyData
}
// impl PrivateKeyResponse {
//     pub fn get_private_key(&self) -> Result<String> {
//         let encrypted_data: Value = serde_json::from_str(&*self.keys.private_key).map_err(|e| Errors::JsonDeserializeError(e))?;
//         let engine = base64::engine::general_purpose::STANDARD;
//         let iv = engine.decode(encrypted_data["iv"].clone()).map_err(|e| Errors::Base64Error(e))?;
//         let cipher = engine.decode(encrypted_data["cipher"].clone()).map_err(|e| Errors::Base64Error(e))?;
//     }
//     pub fn get_public_key(&self) -> String {
//         self.keys.public_key.clone()
//     }
// }
#[derive(Debug, Deserialize)]
pub struct PrivateKeyData {
    pub user_id: String,
    pub r#type: String,
    pub format: String,
    pub private_key: String,
    pub public_key: String,
    pub public_key_signature: String,
    pub time: String,
    pub deleted: Option<Value>,
    pub version: usize,
}
#[derive(Debug, Deserialize)]
pub struct PrivateKey {
    pub private: String
}

#[derive(Serialize)]
pub struct OtherUserInfoRequest {
    client_key: String,
    device_id: String,
    user_id: String,
    withkey: bool,
}
impl OtherUserInfoRequest {
    pub fn new(state: &State, user_id: String, with_key: bool) -> Result<Self> {
        Ok((Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            user_id,
            withkey: with_key,
        }))
    }
}
#[derive(Debug, Deserialize)]
pub struct OtherUserInfoResponse {
    pub user: User,
}
#[derive(Deserialize)]
pub struct PublicSigningKey {
    pub alg: String,
    pub e: String,
    pub ext: bool,
    pub key_ops: Vec<String>,
    pub kty: String,
    pub n: String,
}