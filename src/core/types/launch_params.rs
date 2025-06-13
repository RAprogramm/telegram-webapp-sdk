#[derive(Debug, Clone)]
pub struct LaunchParams {
    pub tg_web_app_platform:      Option<String>,
    pub tg_web_app_version:       Option<String>,
    pub tg_web_app_start_param:   Option<String>,
    pub tg_web_app_show_settings: Option<bool>,
    pub tg_web_app_bot_inline:    Option<bool>
}
