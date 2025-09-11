use std::collections::HashMap;

use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{CssStyleDeclaration, HtmlElement};

use crate::logger::warn;

/// Represents all theme parameters provided by the Telegram WebApp API.
///
/// Each field corresponds to a CSS color value in `#RRGGBB` format.  
/// When deserialized from `Telegram.WebApp.themeParams`, only the colors
/// actually present in the user’s current Telegram theme will be `Some`.
///
/// # Example
///
/// ```ignore
/// use serde_wasm_bindgen::from_value;
/// # use wasm_bindgen::JsValue;
/// # let js_value = /* obtain JS value from Telegram.WebApp.themeParams */
/// let theme: TelegramThemeParams = from_value(js_value)?;
/// theme.apply_to_root()?;
/// # Ok::<(), JsValue>(())
/// ```
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TelegramThemeParams {
    /// Primary background color (`--tg-theme-bg-color`).
    pub bg_color: Option<String>,

    /// Primary text color (`--tg-theme-text-color`).
    pub text_color: Option<String>,

    /// Hint text color (`--tg-theme-hint-color`).
    pub hint_color: Option<String>,

    /// Link color (`--tg-theme-link-color`).
    pub link_color: Option<String>,

    /// Button background color (`--tg-theme-button-color`).
    pub button_color: Option<String>,

    /// Button text color (`--tg-theme-button-text-color`).
    pub button_text_color: Option<String>,

    /// Secondary background color (`--tg-theme-secondary-bg-color`).
    pub secondary_bg_color: Option<String>,

    /// Header background color (`--tg-theme-header-bg-color`).
    pub header_bg_color: Option<String>,

    /// Bottom bar background color (`--tg-theme-bottom-bar-bg-color`).
    pub bottom_bar_bg_color: Option<String>,

    /// Accent text color (`--tg-theme-accent-text-color`).
    pub accent_text_color: Option<String>,

    /// Section background color (`--tg-theme-section-bg-color`).
    pub section_bg_color: Option<String>,

    /// Section header text color (`--tg-theme-section-header-text-color`).
    pub section_header_text_color: Option<String>,

    /// Section separator color (`--tg-theme-section-separator-color`).
    pub section_separator_color: Option<String>,

    /// Subtitle text color (`--tg-theme-subtitle-text-color`).
    pub subtitle_text_color: Option<String>,

    /// Destructive action text color, e.g. “Delete”
    /// (`--tg-theme-destructive-text-color`).
    pub destructive_text_color: Option<String>
}

impl TelegramThemeParams {
    /// Converts all `Some` theme parameters into a map of CSS custom
    /// properties.
    ///
    /// # Returns
    ///
    /// A `HashMap` where each key is a CSS variable name like
    /// `"--tg-theme-bg-color"`, and the corresponding value is the `#RRGGBB`
    /// color string.
    ///
    /// # Examples
    ///
    /// ```
    /// use telegram_webapp_sdk::core::types::theme_params::TelegramThemeParams;
    /// let theme = TelegramThemeParams {
    ///     bg_color: Some("#ffffff".into()),
    ///     text_color: Some("#000000".into()),
    ///     ..Default::default()
    /// };
    /// let vars = theme.into_css_vars();
    /// assert_eq!(
    ///     vars.get("--tg-theme-bg-color"),
    ///     Some(&"#ffffff".to_string())
    /// );
    /// assert_eq!(
    ///     vars.get("--tg-theme-text-color"),
    ///     Some(&"#000000".to_string())
    /// );
    /// ```
    pub fn into_css_vars(self) -> HashMap<String, String> {
        let mut vars: HashMap<String, String> = HashMap::with_capacity(16);
        let mut push = |key: &str, value: Option<String>| {
            if let Some(v) = value {
                vars.insert(format!("--tg-theme-{}", key), v);
            }
        };

        push("bg-color", self.bg_color);
        push("text-color", self.text_color);
        push("hint-color", self.hint_color);
        push("link-color", self.link_color);
        push("button-color", self.button_color);
        push("button-text-color", self.button_text_color);
        push("secondary-bg-color", self.secondary_bg_color);
        push("header-bg-color", self.header_bg_color);
        push("bottom-bar-bg-color", self.bottom_bar_bg_color);
        push("accent-text-color", self.accent_text_color);
        push("section-bg-color", self.section_bg_color);
        push("section-header-text-color", self.section_header_text_color);
        push("section-separator-color", self.section_separator_color);
        push("subtitle-text-color", self.subtitle_text_color);
        push("destructive-text-color", self.destructive_text_color);

        vars
    }

    /// Applies all CSS custom properties to the document’s root element
    /// (`:root`).
    ///
    /// This makes any CSS rules using `var(--tg-theme-…)` automatically adopt
    /// the Telegram user’s current theme colors.
    ///
    /// # Errors
    ///
    /// Returns `Err(JsValue)` if the global `window` or `document` objects are
    /// unavailable or if the document root element cannot be cast to an
    /// `HtmlElement`.
    pub fn apply_to_root(self) -> Result<(), JsValue> {
        let document = web_sys::window()
            .ok_or_else(|| JsValue::from_str("Global `window` object not available"))?
            .document()
            .ok_or_else(|| JsValue::from_str("Global `document` object not available"))?;

        // Cast the <html> element to HtmlElement to call `.style()`
        let html_el: HtmlElement = document
            .document_element()
            .ok_or_else(|| JsValue::from_str("Document root element missing"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| JsValue::from_str("Document root is not an HtmlElement"))?;

        let style: CssStyleDeclaration = html_el.style();
        for (key, val) in self.into_css_vars() {
            style.set_property(&key, &val).unwrap_or_else(|err| {
                // extract a string from the JsValue or fall back to Debug
                let err_msg = err.as_string().unwrap_or_else(|| format!("{:?}", err));
                // log via your styled logger
                warn(&format!(
                    "Failed to set CSS var {} = {}: {}",
                    key, val, err_msg
                ));
            });
        }

        Ok(())
    }

    /// Returns all non‐empty theme parameters as a vector of
    /// `(css_variable_name, color_value)` pairs.
    pub fn to_map(&self) -> Vec<(String, String)> {
        self.clone() // clone our struct
            .into_css_vars() // move into a HashMap<String,String>
            .into_iter() // turn that into an iterator of (String,String)
            .collect() // collect into a Vec
    }
}

#[wasm_bindgen]
pub fn apply_default_theme() -> Result<(), JsValue> {
    let theme: TelegramThemeParams = Default::default();
    theme.apply_to_root()
}
