use telegram_webapp_sdk::{
    logger::info,
    webapp::{BottomButton, TelegramWebApp}
};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{HtmlElement, console, window};

type Handler = fn(&TelegramWebApp) -> Result<(), JsValue>;

/// Button id -> handler mapping
/// Handlers return Result to propagate JS/WASM errors without panicking.
const BUTTON_IDS: &[(&str, Handler)] = &[
    // Sends arbitrary data to Telegram
    ("send-data", |tg| tg.send_data("Hello from Dev Menu!")),
    // Expands the WebApp viewport
    ("expand", |tg| tg.expand()),
    // Closes the WebApp
    ("close", |tg| tg.close()),
    // Shows test alert
    ("alert", |tg| {
        tg.show_alert("This is a test alert from DevMenu")
    }),
    // Touches main bottom button
    ("main-button", |tg| {
        // These setters are infallible in the SDK; wrap in Ok to fit the Handler type.
        let _ = tg.set_bottom_button_text(BottomButton::Main, "Clicked!");
        let _ = tg.show_bottom_button(BottomButton::Main);
        Ok(())
    }),
    // Checks expansion state and acts accordingly
    ("is-expanded", |tg| {
        let expanded = tg.is_expanded();
        info(&format!("isExpanded = {}", expanded));
        if expanded {
            tg.show_alert("Viewport is already expanded.")?;
        } else {
            tg.expand()?;
        }
        Ok(())
    }),
    // Tries to show "Add to Home Screen" prompt and logs the result
    ("add-to-home-screen", |tg| {
        if let Ok(shown) = tg.add_to_home_screen() {
            info(&format!("addToHomeScreen shown = {}", shown));
        }
        Ok(())
    }),
    // Checks home screen status asynchronously
    ("check-home-screen", |tg| {
        let _ = tg.check_home_screen_status(|status| {
            // callback is fire-and-forget; log status
            info(&format!("home screen status: {}", status));
        });
        Ok(())
    })
];

pub fn setup_dev_menu() {
    // Try to get document; silently return if not available (e.g., not in browser)
    let doc = match window().and_then(|w| w.document()) {
        Some(doc) => doc,
        None => return
    };

    for (id, handler) in BUTTON_IDS {
        if let Some(elem) = doc
            .get_element_by_id(id)
            .and_then(|e| e.dyn_into::<HtmlElement>().ok())
        {
            // No need to clone function pointers; they're Copy.
            let handler = *handler;

            // Create JS callback that invokes our handler, logging errors to console
            let cb = Closure::<dyn FnMut()>::new(move || {
                // Collapse nested ifs via let-chains to satisfy clippy::collapsible-if
                if let Some(tg) = TelegramWebApp::instance()
                    && let Err(err) = handler(&tg)
                {
                    // Log JS/WASM error object to developer console
                    console::error_1(&err);
                }
            });

            elem.set_onclick(Some(cb.as_ref().unchecked_ref()));
            cb.forget(); // Intentional leak for dev menu lifetime
        }
    }
}
