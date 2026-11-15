// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Benchmarks for the platform-independent parts of the SDK: `initData`
//! parsing, theme parameter conversion and mock data generation.

use divan::{Bencher, black_box};
use telegram_webapp_sdk::{
    core::types::{
        chat::TelegramChat, init_data::TelegramInitData,
        init_data_internal::TelegramInitDataInternal, theme_params::TelegramThemeParams,
        user::TelegramUser
    },
    mock::{config::MockTelegramConfig, data::MockTelegramUser, utils::generate_mock_init_data}
};

fn main() {
    divan::main();
}

const MOCK_CONFIG: &str = include_str!("../telegram-webapp.toml");

fn sample_user() -> MockTelegramUser {
    MockTelegramUser {
        id:                 279_058_397,
        first_name:         "Vladislav".into(),
        last_name:          Some("Kibenko".into()),
        username:           Some("vdkfrost".into()),
        language_code:      Some("ru".into()),
        is_premium:         Some(true),
        allows_write_to_pm: Some(true)
    }
}

fn sample_chat_json() -> String {
    r#"{"id":1234567890,"type":"supergroup","title":"Rust & WebAssembly enthusiasts","username":"rust_wasm","photo_url":"https://t.me/i/userpic/320/rust_wasm.jpg"}"#
        .to_string()
}

/// Builds a realistic URL-encoded `initData` string containing user,
/// receiver and chat payloads.
fn sample_init_data() -> String {
    let base = generate_mock_init_data(
        &sample_user(),
        "1662771648",
        "c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2",
        Some("AAHdF6IQAAAAAN0XohDhrOrc")
    );
    let receiver = urlencoding::encode(
        r#"{"id":987654321,"is_bot":false,"first_name":"Ana","username":"ana_dev","language_code":"en","photo_url":"https://t.me/i/userpic/320/ana.jpg"}"#
    );
    let chat_json = sample_chat_json();
    let chat = urlencoding::encode(&chat_json);
    format!(
        "{base}&receiver={receiver}&chat={chat}&chat_type=supergroup&chat_instance=-8382547620598470813&start_param=ref_campaign_42&can_send_after=10&signature=qWsA3iUQ0dGwLp5Dy0Gq_9ZkR3yN8bWm1vJ2ZtEaHxCw"
    )
}

fn full_theme() -> TelegramThemeParams {
    TelegramThemeParams {
        bg_color:                  Some("#ffffff".into()),
        text_color:                Some("#000000".into()),
        hint_color:                Some("#888888".into()),
        link_color:                Some("#2689bf".into()),
        button_color:              Some("#0088cc".into()),
        button_text_color:         Some("#ffffff".into()),
        secondary_bg_color:        Some("#f0f0f0".into()),
        header_bg_color:           Some("#1d1f21".into()),
        bottom_bar_bg_color:       Some("#1f2226".into()),
        accent_text_color:         Some("#2eaee3".into()),
        section_bg_color:          Some("#222529".into()),
        section_header_text_color: Some("#c8c9cb".into()),
        section_separator_color:   Some("#2a2c30".into()),
        subtitle_text_color:       Some("#909398".into()),
        destructive_text_color:    Some("#e33e3e".into())
    }
}

/// Mirrors the parsing pipeline used by `try_init_sdk`: URL-decode the raw
/// payload, then deserialize the embedded JSON objects.
fn parse_init_data(raw: &str) -> TelegramInitData {
    let raw: TelegramInitDataInternal = serde_urlencoded::from_str(raw).unwrap();
    let user: Option<TelegramUser> = raw
        .user
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .unwrap();
    let receiver: Option<TelegramUser> = raw
        .receiver
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .unwrap();
    let chat: Option<TelegramChat> = raw
        .chat
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .unwrap();
    TelegramInitData {
        query_id: raw.query_id,
        user,
        receiver,
        chat,
        chat_type: raw.chat_type,
        chat_instance: raw.chat_instance,
        start_param: raw.start_param,
        can_send_after: raw.can_send_after,
        auth_date: raw.auth_date,
        hash: raw.hash,
        signature: raw.signature
    }
}

mod init_data {
    use super::*;

    #[divan::bench]
    fn urlencoded_internal(bencher: Bencher) {
        let raw = sample_init_data();
        bencher.bench(|| {
            let parsed: TelegramInitDataInternal =
                serde_urlencoded::from_str(black_box(&raw)).unwrap();
            black_box(parsed)
        });
    }

    #[divan::bench]
    fn full_pipeline(bencher: Bencher) {
        let raw = sample_init_data();
        bencher.bench(|| black_box(parse_init_data(black_box(&raw))));
    }

    #[divan::bench(args = [1, 10, 100])]
    fn full_pipeline_batch(bencher: Bencher, n: usize) {
        let payloads: Vec<String> = (0..n).map(|_| sample_init_data()).collect();
        bencher.bench(|| {
            for raw in &payloads {
                black_box(parse_init_data(black_box(raw)));
            }
        });
    }

    #[divan::bench]
    fn user_json(bencher: Bencher) {
        let json = serde_json::to_string(&sample_user()).unwrap();
        bencher.bench(|| {
            let user: TelegramUser = serde_json::from_str(black_box(&json)).unwrap();
            black_box(user)
        });
    }

    #[divan::bench]
    fn chat_json(bencher: Bencher) {
        let json = sample_chat_json();
        bencher.bench(|| {
            let chat: TelegramChat = serde_json::from_str(black_box(&json)).unwrap();
            black_box(chat)
        });
    }
}

mod theme {
    use super::*;

    #[divan::bench]
    fn css_vars_full(bencher: Bencher) {
        let theme = full_theme();
        bencher.bench(|| black_box(black_box(&theme).css_vars()));
    }

    #[divan::bench]
    fn css_vars_empty(bencher: Bencher) {
        let theme = TelegramThemeParams::default();
        bencher.bench(|| black_box(black_box(&theme).css_vars()));
    }

    #[divan::bench]
    fn into_css_vars(bencher: Bencher) {
        bencher
            .with_inputs(full_theme)
            .bench_values(|theme| black_box(theme.into_css_vars()));
    }

    #[divan::bench]
    fn to_map(bencher: Bencher) {
        let theme = full_theme();
        bencher.bench(|| black_box(black_box(&theme).to_map()));
    }
}

mod mock {
    use super::*;

    #[divan::bench]
    fn generate_init_data(bencher: Bencher) {
        let user = sample_user();
        bencher.bench(|| {
            black_box(generate_mock_init_data(
                black_box(&user),
                "1662771648",
                "c501b71e775f74ce10e377dea85a7ea24ecd640b",
                Some("AAHdF6IQAAAAAN0XohDhrOrc")
            ))
        });
    }

    #[divan::bench]
    fn parse_config_toml(bencher: Bencher) {
        bencher.bench(|| {
            let cfg: MockTelegramConfig = toml::from_str(black_box(MOCK_CONFIG)).unwrap();
            black_box(cfg)
        });
    }
}
