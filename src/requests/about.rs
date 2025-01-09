use crate::errors::Errors;
use crate::request_types::about;
use crate::requests::post_request;
use crate::state::{EncryptionState, State};
use crate::types::user::companies::Company;
use crate::types::user::general::UserInfo;
use crate::Result;
use openssl::rsa::Rsa;
use crate::types::user::others::User;

pub async fn get_user_info(state: &State) -> Result<UserInfo> {
    Ok(post_request::<about::UserInfoResponse>(state, "/users/me", about::UserInfoRequest::new(state, false)?).await?.user)
}
pub async fn get_companies(state: &State) -> Result<Vec<Company>> {
    Ok(post_request::<about::CompanyResponse>(state, "/company/member", about::CompanyRequest::new(state)?).await?.companies)
}
pub async fn get_encryption_state(state: &State, passphrase: String) -> Result<EncryptionState> {
    let encrypt = post_request::<about::PrivateKeyResponse>(state,
                                                            "/security/get_private_key",
                                                            about::PrivateKeyRequest::new(state, "pem", "encryption")?)
        .await?;
    // let sign = post_request::<about::PrivateKeyResponse>(state,
    //                                                      "/security/get_private_key",
    //                                                      about::PrivateKeyRequest::new(state, "pem", "signing")?)
    //     .await?;
    let private_encrypt = serde_json::from_str::<about::PrivateKey>(&*encrypt.keys.private_key).map_err(|e| Errors::JsonDeserializeError(e))?.private;
    // let private_sign = about::PrivateKey::deserialize(sign.keys.private_key.into_deserializer()).map_err(|e| Errors::JsonDeserializeError(e))?.private;
    let private_encrypt = Rsa::private_key_from_pem_passphrase(private_encrypt.as_ref(), passphrase.as_ref()).map_err(|e| Errors::EncryptionError(e))?;
    let public_encrypt = Rsa::public_key_from_pem(encrypt.keys.public_key.as_ref()).map_err(|e| Errors::EncryptionError(e))?;
    // let private_sign = Rsa::private_key_from_pem_passphrase(private_sign.as_ref(), passphrase.as_ref()).map_err(|e| Errors::EncryptionError(e))?;
    // let public_sign = Rsa::public_key_from_pem(sign.keys.public_key.as_ref()).map_err(|e| Errors::EncryptionError(e))?;
    Ok(EncryptionState::new(private_encrypt, public_encrypt))
}
pub async fn get_other_user_info(state: &State, user_id: String) -> Result<User> {
    Ok(post_request::<about::OtherUserInfoResponse>(state,
                                                    "/users/info",
                                                    about::OtherUserInfoRequest::new(state, user_id, true)?)
        .await?.user)
}