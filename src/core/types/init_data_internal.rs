use serde::Deserialize;

#[derive(Deserialize)]
pub struct TelegramInitDataInternal {
    pub user:           Option<String>,
    pub receiver:       Option<String>,
    pub chat:           Option<String>,
    pub chat_type:      Option<String>,
    pub chat_instance:  Option<String>,
    pub start_param:    Option<String>,
    pub can_send_after: Option<u64>,
    pub auth_date:      u64,
    pub hash:           String,
    pub signature:      Option<String>
}
