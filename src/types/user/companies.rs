use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::user::general::{AccessRestrictions, PasswordRestrictions, SettingsMDM};
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub quota: String,
    pub max_users: Option<Value>,
    pub created: String,
    pub deleted: Option<Value>,
    pub online_payment: String,
    pub freemium: String,
    pub logo: String,
    pub logo_url: String,
    pub users: CompanyUsers,
    pub features: Vec<String>,
    pub marketplace_modules: Vec<String>,
    pub provider: String,
    pub protected: bool,
    pub roles: Vec<crate::types::user::general::UserRole>,
    pub permissions: Option<Vec<String>>,
    pub settings: CompanySettings,
    pub domains: Vec<Value>,
    pub domain: Option<String>,
    pub time_joined: String,
    pub membership_expiry: Option<Value>,
    pub deactivated: Option<Value>,
    pub maps: Vec<Value>,
    pub unread_messages: usize
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CompanyUsers {
    pub created: usize,
    pub active: usize,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CompanySettings {
    pub device_pin: String,
    pub device_pin_delay: String,
    pub device_gps: String,
    pub device_encryption: String,
    pub file_export: bool,
    pub file_import: bool,
    pub share_links: bool,
    pub encryption: bool,
    pub open_channels: bool,
    pub autostart: bool,
    pub lockscreen_content: String,
    pub client_count: usize,
    pub email_validation: String,
    pub may_change_email: bool,
    pub may_change_password: bool,
    pub manual_account_creation: bool,
    pub ttl_content: Option<Value>,
    pub ttl_marked_content: Option<Value>,
    pub ttl_server_content: Option<Value>,
    pub can_delete_messages: bool,
    pub share_unencrypted_files_into_encrypted_chats: bool,
    pub force_device_notifications: bool,
    pub device_login_management: bool,
    pub self_deletion: bool,
    pub language: String,
    pub mdm: SettingsMDM,
    pub membership_expired_notify_1: String,
    pub membership_expired_notify_2: String,
    pub membership_expired_notify_3: String,
    pub waiting_period_days: usize,
    pub ldapsync_enabled: bool,
    pub ldapsync_usersync_only: bool,
    pub link_preview: bool,
    pub password_restrictions: PasswordRestrictions,
    pub can_block_users: bool,
    pub can_report_users: bool,
    pub can_report_messages: bool,
    pub access_restrictions: AccessRestrictions,
}