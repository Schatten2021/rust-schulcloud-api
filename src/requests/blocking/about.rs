use crate::errors::Errors;
use crate::request_types::about;
use crate::request_types::about::{EncryptedPrivateKeyData, RSAPrivateKey, RSAPublicKey};
use crate::requests::blocking::post_request;
use crate::state::{EncryptionState, State};
use crate::types::user::companies::Company;
use crate::types::user::general::UserInfo;
use crate::types::user::others::User;
use crate::Result;
use base64::Engine;
use openssl::hash::MessageDigest;
use openssl::rsa::Padding;
use openssl::symm::Cipher;

const BASE64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

pub fn get_user_info(state: &State) -> Result<UserInfo> {
    Ok(post_request::<about::UserInfoResponse>(state, "/users/me", about::UserInfoRequest::new(state, false)?)?.user)
}
pub fn get_companies(state: &State) -> Result<Vec<Company>> {
    Ok(post_request::<about::CompanyResponse>(state, "/company/member", about::CompanyRequest::new(state)?)?.companies)
}
pub fn get_encryption_state(state: &State, passphrase: String) -> Result<EncryptionState> {
    // load the encryption key
    let encrypt = post_request::<about::PrivateKeyResponse>(state,
                                                            "/security/get_private_key",
                                                            about::PrivateKeyRequest::new(state, "jwk", "encryption")?)?;
    // private key info is stored as a string containing JSON data.
    let encrypted_private_key_info: EncryptedPrivateKeyData = serde_json::from_str(&*encrypt.keys.private_key)?;

    // derive key decryption AES key
    let derivation_properties = encrypted_private_key_info.key_derivation_properties
        .ok_or(Errors::ValueError("API didn't respond with key derivation properties".to_string()))?;
    let mut derived_key = vec![0;32];
    let salt = BASE64.decode(derivation_properties.salt)?;
    openssl::pkcs5::pbkdf2_hmac(passphrase.as_bytes(), &*salt, derivation_properties.iterations, MessageDigest::sha256(), &mut *derived_key)?;

    // load the encryption keys
    let iv = BASE64.decode(encrypted_private_key_info.iv)?;
    let encrypted_private_encrypt = BASE64.decode(encrypted_private_key_info.ciphertext)?;
    let decrypted_encrypt = openssl::symm::decrypt(Cipher::aes_256_cbc(), &*derived_key, Some(&*iv), &*encrypted_private_encrypt)?;
    let private_encrypt = RSAPrivateKey::from_decrypted(decrypted_encrypt)?.to_key()?;
    let public_encrypt = RSAPublicKey::from_str(&*encrypt.keys.public_key)?.to_key()?;

    // load signing key
    let sign = post_request::<about::PrivateKeyResponse>(state,
                                                         "/security/get_private_key",
                                                         about::PrivateKeyRequest::new(state, "jwk", "signing")?)?;
    let encrypted_sing: EncryptedPrivateKeyData = serde_json::from_str(&*sign.keys.private_key).map_err(|e| Errors::JsonDeserializeError(e))?;
    let encrypted_kek = encrypted_sing.encryptedKEK.ok_or(Errors::ValueError("No Key Encryption Key (KEK)".to_string()))?;
    let encrypted_kek = BASE64.decode(encrypted_kek)?;

    // decrypt kek
    let mut decrypted_kek = vec![0; private_encrypt.size() as usize];
    if private_encrypt.private_decrypt(&*encrypted_kek, &mut *decrypted_kek, Padding::PKCS1_OAEP)? != 32 {
        return Err(Errors::ValueError("decrypted AES key is not 256 bits long".to_string()));
    };
    decrypted_kek.truncate(32);

    //decrypt RSA key
    let iv = BASE64.decode(encrypted_sing.iv)?;
    let encrypted_signing_key = BASE64.decode(encrypted_sing.ciphertext)?;
    let decrypted_signing_key = openssl::symm::decrypt(Cipher::aes_256_cbc(), &*decrypted_kek, Some(&*iv), &*encrypted_signing_key)?;
    let private_sign= RSAPrivateKey::from_decrypted(decrypted_signing_key)?.to_key()?;
    let public_sign = RSAPublicKey::from_str(&*sign.keys.public_key)?.to_key()?;
    Ok(EncryptionState::new(private_encrypt, public_encrypt, private_sign, public_sign))
}
pub fn get_other_user_info(state: &State, user_id: String) -> Result<User> {
    Ok(post_request::<about::OtherUserInfoResponse>(state,
                                                    "/users/info",
                                                    about::OtherUserInfoRequest::new(state, user_id, true)?)
        ?.user)
}