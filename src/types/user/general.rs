use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct UserInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub socket_id: String,
    pub online: bool,
    pub status: Value,
    pub user_status: Vec<Value>,
    pub active: String,
    pub deleted: Value,
    pub allows_voip_calls: bool,
    pub enter_is_newline: bool,
    pub mx_user_id: String,
    pub federated: bool,
    pub email: String,
    pub email_validated: String,
    pub notifications: bool,
    pub device_notifications: bool,
    pub last_login: String,
    pub language: String,
    pub image: String,
    pub quota: String,
    pub ldap_login: String,
    pub public_key: String,
    pub public_key_signature: String,
    pub public_signing_key: String,
    pub public_key_ca_signature: String,
    pub roles: Vec<UserRole>,
    pub permissions: Vec<String>,
    pub company_features: Vec<String>,
    pub notification_count: u32,
    pub device_id: String,
    pub app_name: String,
    pub push_service: Option<Value>,
    pub push_id: Option<Value>,
    pub ios_voip_push_id: Option<Value>,
    pub settings: UserSettings,
    pub is_bot: bool,
    pub marketplace_modules: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct UserRole {
    pub id: String,
    pub name: String,
    pub global: String,
    pub company_id: String,
    pub time: String,
    pub editable: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct UserSettings {
    pub device_pin: String,
    pub device_pin_delay: PossibleDevicePinDelayOptions,
    pub device_gps: String,
    pub device_encryption: String,
    pub file_export: bool,
    pub file_import: bool,
    pub share_links: bool,
    pub encryption: bool,
    pub open_channels: bool,
    pub autostart: bool,
    pub lockscreen_content: String,
    pub client_count: Value,
    pub email_validation: String,
    pub may_change_email: bool,
    pub may_change_password: bool,
    pub manual_account_creation: bool,
    pub ttl_content: Value,
    pub ttl_marked_content: Value,
    pub ttl_server_content: Value,
    pub can_delete_messages: bool,
    pub share_unencrypted_files_into_encrypted_chats: bool,
    pub force_device_notifications: bool,
    pub device_login_management: bool,
    pub self_deletion: bool,
    pub link_preview: bool,
    pub password_restrictions: PasswordRestrictions,
    pub mdm: SettingsMDM,
    pub can_block_users: bool,
    pub can_report_users: bool,
    pub can_report_messages: bool,
    pub access_restrictions: AccessRestrictions,
    pub translate_messages: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum PossibleDevicePinDelayOptions {
    Int(u64),
    String(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct AccessRestrictions {
    pub blacklisted: Vec<Value>,
    pub whitelisted: Vec<Value>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct SettingsMDM {
    pub mdm_access_calendar: bool,
    pub mdm_access_camera: bool,
    pub mdm_access_gps: bool,
    pub mdm_access_microphone: bool,
    pub mdm_access_storage_pictures: bool,
    pub mdm_access_storage_videos: bool,
    pub mdm_ability_copypaste: bool,
    pub mdm_ability_sharing: bool,
    pub mdm_ability_chat_history: bool,
    pub mdm_access_attachments: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PasswordRestrictions {
    pub pw_restrictions: bool,
    pub pw_min_length: u16,
    pub pw_uppercase_lowercase: bool,
    pub pw_specialchars: bool,
    pub pw_numbers: bool,
}