use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub active: String,
    pub allows_voip_calls: bool,
    pub company_features: Vec<String>,
    pub deleted: Value,
    pub device_notifications: bool,
    pub email: String,
    pub email_validated: String,
    pub enter_is_newline: bool,
    pub federated: bool,
    pub first_name: String,
    pub id: String,
    pub image: String,
    pub language: String,
    pub last_login: String,
    pub last_name: String,
    pub ldap_login: String,
    pub mx_user_id: String,
    pub notifications: bool,
    pub online: bool,
    pub permissions: Vec<String>,
    pub public_key: String,
    pub public_key_ca_signature: String,
    pub public_key_signature: String,
    pub public_signing_key: String,
    pub quota: String,
    pub roles: Vec<UserRole>,
    pub settings: UserSettings,
    pub socket_id: String,
    pub status: Value,
    pub user_status: Vec<Value>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UserRole {
    pub company_id: String,
    pub editable: bool,
    pub global: String,
    pub id: String,
    pub name: String,
    pub time: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub access_restrictions: UserSettingsAccessRestrictions,
    pub autostart: bool,
    pub can_block_users: bool,
    pub can_delete_messages: bool,
    pub can_report_messages: bool,
    pub can_report_users: bool,
    pub client_count: Value,
    pub device_encryption: String,
    pub device_gps: String,
    pub device_login_management: bool,
    pub device_pin: String,
    pub device_pin_delay: PossibleDevicePinDelayOptions,
    pub email_validation: String,
    pub encryption: bool,
    pub file_export: bool,
    pub file_import: bool,
    pub force_device_notifications: bool,
    pub giphy: bool,
    pub link_preview: bool,
    pub lockscreen_content: String,
    pub manual_account_creation: bool,
    pub may_change_email: bool,
    pub may_change_password: bool,
    pub mdm: UserSettingsMDM,
    pub open_channels: bool,
    pub password_restrictions: UserSettingsPasswordRestrictions,
    pub self_deletion: bool,
    pub share_links: bool,
    pub share_unencrypted_files_into_encrypted_chats: bool,
    pub translate_messages: bool,
    pub ttl_content: Value,
    pub ttl_marked_content: Value,
    pub ttl_server_content: Value,
}
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum PossibleDevicePinDelayOptions {
    Int(u64),
    String(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserSettingsAccessRestrictions {
    pub blacklisted: Vec<Value>,
    pub whitelisted: Vec<Value>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UserSettingsMDM {
    pub mdm_ability_chat_history: bool,
    pub mdm_ability_copypaste: bool,
    pub mdm_ability_sharing: bool,
    pub mdm_access_attachments: bool,
    pub mdm_access_calendar: bool,
    pub mdm_access_camera: bool,
    pub mdm_access_gps: bool,
    pub mdm_access_microphone: bool,
    pub mdm_access_storage_pictures: bool,
    pub mdm_access_storage_videos: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UserSettingsPasswordRestrictions {
    pub pw_min_length: u8,
    pub pw_numbers: bool,
    pub pw_restrictions: bool,
    pub pw_specialchars: bool,
    pub pw_uppercase_lowercase: bool,
}