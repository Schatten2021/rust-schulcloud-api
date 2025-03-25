#![allow(dead_code)]
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};
use crate::Result;
use rand::distr::Alphanumeric;
use rand::Rng;
use crate::errors::Errors;

#[derive(Debug, Clone)]
pub struct EncryptionState {
    private_key: Rsa<Private>,
    public_key: Rsa<Public>,
    private_signing_key: Rsa<Private>,
    public_signing_key: Rsa<Public>,
}
impl EncryptionState {
    pub fn new(private_encryption_key: Rsa<Private>, public_encryption_key: Rsa<Public>, private_signing_key: Rsa<Private>, public_signing_key: Rsa<Public>) -> Self {
        Self {
            private_key: private_encryption_key,
            public_key: public_encryption_key,
            private_signing_key,
            public_signing_key
        }
    }
    pub fn decrypt(&self, key: String) -> Result<Vec<u8>> {
        let encrypted_data = BASE64.decode(key).map_err(|e| Errors::Base64Error(e))?;
        let mut result = vec![0; encrypted_data.len()];
        let decrypted_len = self.private_key.private_decrypt(&*encrypted_data, &mut *result, Padding::PKCS1_OAEP).map_err(|e| Errors::EncryptionError(e))?;
        result.truncate(decrypted_len);
        Ok(result)
    }
    pub fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        let mut result = vec![0; self.private_key.size() as usize];
        let encrypted_len = self.private_key.private_encrypt(&*data, &mut *result, Padding::PKCS1_OAEP).map_err(|e| Errors::EncryptionError(e))?;
        result.truncate(encrypted_len);
        Ok(result)
    }
    #[cfg(feature = "experimental")]
    pub fn sign(&self, data: Vec<u8>) -> Result<String> {
        let key = PKey::from_rsa(self.private_key.clone())?;
        let mut signer = openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(), &key)?;
        signer.update(&*data)?;
        Ok(BASE64.encode(signer.sign_to_vec()?))
    }
}
#[derive(Clone, Debug)]
pub struct State {
    pub(crate) base_url: String,
    pub device_id: String,
    pub client_key: Option<String>,
}
impl State {
    pub fn build_url(&self, path: impl ToString) -> String {
        let path = path.to_string();
        let base = if self.base_url.ends_with('/') {
            self.base_url.clone()
        } else {
            self.base_url.clone() + "/"
        };
        base + path.trim_start_matches("/")
    }
    pub(crate) fn expect_client_key(&self) -> Result<String> {
        self.client_key.clone().ok_or(Errors::NotAuthenticated)
    }
    pub(crate) fn get_device_id(&self) -> String {self.device_id.clone()}
}
impl State {
    pub fn new(base_url: String, device_id: String, client_key: Option<String>) -> Self {
        let base_url = if base_url.starts_with("http://") || base_url.starts_with("https://") {
            base_url
        } else {
            "https://".to_string() + &base_url
        };
        Self {
            base_url,
            device_id,
            client_key,
        }
    }
}
impl Default for State {
    fn default() -> Self {
        Self {
            base_url: "https://api.stashcat.com/".into(),
            device_id: rand::rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect(),
            client_key: None,
        }
    }
}