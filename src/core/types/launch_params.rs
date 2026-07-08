// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

/// Launch parameters parsed from the Mini App URL query string.
///
/// Telegram appends a set of `tgWebApp*` query parameters to the URL it opens
/// for the Mini App. This struct captures the subset used to determine the host
/// platform, API version and launch options.
#[derive(Debug, Clone)]
pub struct LaunchParams {
    /// Host platform the Mini App is running on, from `tgWebAppPlatform`
    /// (e.g. `"android"`, `"ios"`, `"tdesktop"`, `"web"`).
    pub tg_web_app_platform:      Option<String>,
    /// Version of the Telegram WebApp API supported by the host, from
    /// `tgWebAppVersion`.
    pub tg_web_app_version:       Option<String>,
    /// Deep-link start parameter passed to the Mini App, from
    /// `tgWebAppStartParam`.
    pub tg_web_app_start_param:   Option<String>,
    /// Whether the settings button should be shown, parsed from
    /// `tgWebAppShowSettings` (`"1"` maps to `true`).
    pub tg_web_app_show_settings: Option<bool>,
    /// Whether the Mini App was launched in inline mode from the bot, parsed
    /// from `tgWebAppBotInline` (`"1"` maps to `true`).
    pub tg_web_app_bot_inline:    Option<bool>
}
