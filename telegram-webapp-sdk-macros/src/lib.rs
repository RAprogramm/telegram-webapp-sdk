use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn telegram_page(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let expanded = quote! {
        #[::inventory::submit(::telegram_webapp_sdk::pages::Page { path: #path, handler: #name })]
        #input
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn telegram_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;
    let expanded = quote! {
        #[::wasm_bindgen::prelude::wasm_bindgen(start)]
        #(#attrs)*
        #vis #sig {
            if !::telegram_webapp_sdk::utils::check_env::is_telegram_env() {
                #[cfg(debug_assertions)]
                if let Ok(cfg) = ::telegram_webapp_sdk::mock::config::MockTelegramConfig::from_file("telegram-webapp.toml") {
                    let _ = ::telegram_webapp_sdk::mock::init::mock_telegram_webapp(cfg);
                }
            }
            ::telegram_webapp_sdk::core::init::init_sdk()?;
            #block
        }
    };
    expanded.into()
}

#[proc_macro]
pub fn telegram_router(_item: TokenStream) -> TokenStream {
    let expanded = quote! {
        {
            let mut router = Router::new();
            for page in ::telegram_webapp_sdk::pages::iter() {
                router = router.register(page.path, page.handler);
            }
            router.start();
        }
    };
    expanded.into()
}
