use super::data::MockTelegramUser;

#[derive(Default)]
pub struct MockTelegramConfig {
    pub user: Option<MockTelegramUser>,
    pub auth_date: Option<String>,
    pub hash: Option<String>,
    pub bg_color: Option<String>,
    pub text_color: Option<String>,
    pub hint_color: Option<String>,
    pub link_color: Option<String>,
    pub button_color: Option<String>,
    pub button_text_color: Option<String>,
    pub secondary_bg_color: Option<String>,
    pub header_bg_color: Option<String>,
    pub bottom_bar_bg_color: Option<String>,
    pub accent_text_color: Option<String>,
    pub section_bg_color: Option<String>,
    pub section_header_text_color: Option<String>,
    pub section_separator_color: Option<String>,
    pub subtitle_text_color: Option<String>,
    pub destructive_text_color: Option<String>,
    pub platform: Option<String>,
    pub version: Option<String>
}
