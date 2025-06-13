use super::data::MockTelegramUser;

#[derive(Default)]
pub struct MockTelegramConfig {
    pub user:       Option<MockTelegramUser>,
    pub auth_date:  Option<String>,
    pub hash:       Option<String>,
    pub bg_color:   Option<String>,
    pub text_color: Option<String>,
    pub platform:   Option<String>,
    pub version:    Option<String>
}
