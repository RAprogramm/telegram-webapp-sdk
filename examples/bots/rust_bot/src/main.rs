// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::{AppError, AppErrorKind};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo},
    utils::command::BotCommands
};
use webapp_bot_example::OrderData;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let webapp_url = std::env::var("WEBAPP_URL")
        .unwrap_or_else(|_| "https://example.com/index.html".to_string());

    tracing::info!("Starting WebApp bot with URL: {}", webapp_url);

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_command)
        )
        .branch(
            dptree::filter(|msg: Message| msg.web_app_data().is_some())
                .endpoint(handle_webapp_data)
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

/// Bot commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display welcome message with WebApp buttons
    #[command(description = "Display welcome message")]
    Start,
    /// Show help information
    #[command(description = "Show help information")]
    Help
}

/// Handles bot commands (/start, /help)
///
/// Sends WebApp buttons for /start or help information for /help
async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> Result<(), AppError> {
    let webapp_url = std::env::var("WEBAPP_URL")
        .unwrap_or_else(|_| "https://example.com/index.html".to_string());

    match cmd {
        Command::Start => {
            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![InlineKeyboardButton::web_app(
                    "Open Burger King Menu",
                    WebAppInfo {
                        url: format!("{}#/burger-king", webapp_url)
                            .parse()
                            .map_err(|e| {
                                AppError::new(AppErrorKind::Internal, "Invalid WebApp URL")
                                    .with_context(e)
                            })?
                    }
                )],
                vec![InlineKeyboardButton::web_app(
                    "View Init Data",
                    WebAppInfo {
                        url: format!("{}#/init-data", webapp_url).parse().map_err(|e| {
                            AppError::new(AppErrorKind::Internal, "Invalid WebApp URL")
                                .with_context(e)
                        })?
                    }
                )],
                vec![InlineKeyboardButton::web_app(
                    "Theme Parameters",
                    WebAppInfo {
                        url: format!("{}#/theme-params", webapp_url)
                            .parse()
                            .map_err(|e| {
                                AppError::new(AppErrorKind::Internal, "Invalid WebApp URL")
                                    .with_context(e)
                            })?
                    }
                )],
            ]);

            bot.send_message(
                msg.chat.id,
                "Welcome to Telegram WebApp SDK Demo!\n\nClick a button to open the WebApp:"
            )
            .reply_markup(keyboard)
            .await
            .map_err(|e| {
                AppError::new(AppErrorKind::Service, "Failed to send message").with_context(e)
            })?;
        }
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                "This bot demonstrates telegram-webapp-sdk.\n\n\
                 Commands:\n\
                 /start - Open WebApp menu\n\
                 /help - Show this message\n\n\
                 GitHub: https://github.com/RAprogramm/telegram-webapp-sdk"
            )
            .await
            .map_err(|e| {
                AppError::new(AppErrorKind::Service, "Failed to send message").with_context(e)
            })?;
        }
    }

    Ok(())
}

/// Handles data received from WebApp
///
/// Processes orders from the Burger King demo and sends confirmation messages
async fn handle_webapp_data(bot: Bot, msg: Message) -> Result<(), AppError> {
    if let Some(web_app_data) = msg.web_app_data() {
        let order: OrderData = serde_json::from_str(&web_app_data.data).map_err(|e| {
            AppError::new(AppErrorKind::BadRequest, "Invalid order data format").with_context(e)
        })?;
        let price_dollars = order.price_cents as f64 / 100.0;

        let response = format!(
            "✅ Order Received!\n\n\
             Item: {}\n\
             Price: ${:.2}\n\
             Order ID: #{}\n\n\
             Your order is being processed...",
            order.name, price_dollars, order.id
        );

        bot.send_message(msg.chat.id, response).await.map_err(|e| {
            AppError::new(AppErrorKind::Service, "Failed to send message").with_context(e)
        })?;

        tracing::info!(
            "Order from user {}: {} (${:.2})",
            msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
            order.name,
            price_dollars
        );
    }

    Ok(())
}
