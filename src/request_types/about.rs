use crate::errors::Errors;
use crate::state::State;
use crate::types::user::general::UserInfo;
use crate::types::user::others::User;
use crate::Result;
use base64::Engine;
use openssl::bn::BigNum;
use openssl::pkey::{Private, Public};
use openssl::rsa::Rsa;
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
#[derive(Debug, Deserialize)]
pub struct PrivateKeyData {
    pub user_id: String,
    pub r#type: String,
    pub format: String,
    pub private_key: String,
    pub public_key: String,
    pub public_key_signature: Option<String>,
    pub time: String,
    pub deleted: Option<Value>,
    pub version: usize,
}
#[derive(Debug, Deserialize)]
pub struct EncryptedPrivateKeyData {
    pub iv: String,
    pub ciphertext: String,
    pub encryption_func: String,
    pub key_derivation_properties: Option<KeyDerivationProperties>,
    pub encryptedKEK: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct KeyDerivationProperties {
    pub prf: String,
    pub iterations: usize,
    pub salt: String,
}
#[derive(Debug, Deserialize)]
pub struct PemPrivateKey {
    pub private: String
}
#[derive(Debug, Deserialize)]
pub struct RSAPrivateKey {
    pub n: String,
    pub e: String,
    pub d: String,
    pub p: String,
    pub q: String,
    pub dp: String,
    pub dq: String,
    pub qi: String,
}
impl From<RSAPrivateKey> for Result<Rsa<Private>> {
    fn from(value: RSAPrivateKey) -> Self {
        (&value).into()
    }
}
impl From<&RSAPrivateKey> for Result<Rsa<Private>> {
    fn from(value: &RSAPrivateKey) -> Self {
        Ok(openssl::rsa::RsaPrivateKeyBuilder::new(
            to_big_num(value.n.clone())?,
            to_big_num(value.e.clone())?,
            to_big_num(value.d.clone())?,
        )?.set_factors(
            to_big_num(value.p.clone())?,
            to_big_num(value.q.clone())?,
        )?.build())
    }
}
impl RSAPrivateKey {
    pub fn from_str(private_key: &str) -> Result<RSAPrivateKey> {
        serde_json::from_str(private_key).map_err(Errors::from)
    }
    pub fn from_decrypted(private_key: Vec<u8>) -> Result<RSAPrivateKey> {
        Self::from_str(&*String::from_utf8(private_key)?)
    }
    pub fn to_key(&self) -> Result<Rsa<Private>> {
        self.into()
    }
}
fn to_big_num(string: String) -> Result<BigNum> {
    BigNum::from_slice(&*base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(string)
        .map_err(|e| Errors::Base64Error(e))?)
        .map_err(|e| Errors::EncryptionError(e))
}
#[derive(Debug, Deserialize)]
pub struct RSAPublicKey {
    pub n: String,
    pub e: String,
}
impl RSAPublicKey {
    pub fn from_str(public_key: &str) -> Result<RSAPublicKey> {
        serde_json::from_str(public_key).map_err(|e| Errors::from(e))
    }
    pub fn to_key(&self) -> Result<Rsa<Public>> {
        self.into()
    }
}
impl From<&RSAPublicKey> for Result<Rsa<Public>> {
    fn from(value: &RSAPublicKey) -> Self {
        openssl::rsa::Rsa::from_public_components(
            to_big_num(value.n.clone())?,
            to_big_num(value.e.clone())?,
        ).map_err(|e| Errors::from(e))
    }
}
impl From<RSAPublicKey> for Result<Rsa<Public>> {
    fn from(value: RSAPublicKey) -> Self {
        (&value).to_key()
    }
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
        Ok(Self {
            client_key: state.expect_client_key()?,
            device_id: state.get_device_id(),
            user_id,
            withkey: with_key,
        })
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