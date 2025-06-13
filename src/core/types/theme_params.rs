use serde::Deserialize;

/// Describes the current theme parameters used in the Telegram client.
#[derive(Deserialize, Debug)]
pub struct TelegramThemeParams {
    /// Background color (#RRGGBB).
    pub bg_color: Option<String>,

    /// Main text color.
    pub text_color: Option<String>,

    /// Hint text color.
    pub hint_color: Option<String>,

    /// Link color.
    pub link_color: Option<String>,

    /// Button background color.
    pub button_color: Option<String>,

    /// Button text color.
    pub button_text_color: Option<String>,

    /// Secondary background color.
    pub secondary_bg_color: Option<String>,

    /// Header background color.
    pub header_bg_color: Option<String>,

    /// Bottom bar background color.
    pub bottom_bar_bg_color: Option<String>,

    /// Accent text color.
    pub accent_text_color: Option<String>,

    /// Background color for section containers.
    pub section_bg_color: Option<String>,

    /// Section header text color.
    pub section_header_text_color: Option<String>,

    /// Section separator color.
    pub section_separator_color: Option<String>,

    /// Subtitle text color.
    pub subtitle_text_color: Option<String>,

    /// Destructive action text color (e.g. "Delete").
    pub destructive_text_color: Option<String>
}
