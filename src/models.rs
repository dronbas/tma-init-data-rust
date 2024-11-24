use serde::Deserialize;

/// Contains launch parameters data
#[derive(Debug, PartialEq, Deserialize)]
pub struct InitData {
    pub auth_date: u64,
    pub can_send_after: Option<u64>,
    pub chat: Option<Chat>,
    pub chat_type: Option<String>,
    pub chat_instance: Option<i64>,
    pub hash: String,
    pub query_id: Option<String>,
    pub receiver: Option<User>,
    pub start_param: Option<String>,
    pub user: Option<User>,
}

/// Describes user information
#[derive(Debug, PartialEq, Deserialize)]
pub struct User {
    pub added_to_attachment_menu: Option<bool>,
    pub allows_write_to_pm: Option<bool>,
    pub is_premium: Option<bool>,
    pub first_name: String,
    pub id: i64,
    pub is_bot: Option<bool>,
    pub last_name: Option<String>,
    pub language_code: Option<String>,
    pub photo_url: Option<String>,
    pub username: Option<String>,
}

/// Describes the chat information
#[derive(Debug, PartialEq, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    pub title: String,
    pub photo_url: Option<String>,
    pub username: Option<String>,
}
