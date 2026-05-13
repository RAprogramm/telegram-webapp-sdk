// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::env;

use masterror::{AppError, AppErrorKind};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo},
    utils::command::BotCommands
};
use webapp_integration_backend::{WebAppMessage, WebAppResponse};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let webapp_url = env::var("WEBAPP_URL").unwrap_or_else(|_| "https://example.com".to_string());

    tracing::info!("Starting WebApp integration bot with URL: {}", webapp_url);

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
async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> Result<(), AppError> {
    let webapp_url = env::var("WEBAPP_URL").unwrap_or_else(|_| "https://example.com".to_string());

    match cmd {
        Command::Start => {
            let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::web_app(
                "Open Integration Demo",
                WebAppInfo {
                    url: webapp_url.to_string().parse().map_err(|e| {
                        AppError::new(AppErrorKind::Internal, "Invalid WebApp URL").with_context(e)
                    })?
                }
            )]]);

            bot.send_message(
                msg.chat.id,
                "Welcome to Telegram WebApp SDK Integration Demo!\n\nClick the button to open the WebApp:"
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
                "This bot demonstrates telegram-webapp-sdk integration with teloxide.\n\n\
                Commands:\n\
                /start - Open WebApp\n\
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

/// Handles data received from WebApp via sendData
async fn handle_webapp_data(bot: Bot, msg: Message) -> Result<(), AppError> {
    if let Some(web_app_data) = msg.web_app_data() {
        // Parse the JSON data sent from WebApp
        let webapp_msg: WebAppMessage = serde_json::from_str(&web_app_data.data).map_err(|e| {
            AppError::new(AppErrorKind::BadRequest, "Invalid WebApp message format")
                .with_context(e)
        })?;

        // Process the message based on action
        let response = match webapp_msg.action.as_str() {
            "echo" => {
                let payload = webapp_msg.payload.clone().unwrap_or_default();
                WebAppResponse {
                    success: true,
                    message: "Echo response from bot".to_string(),
                    data:    Some(format!("You sent: {}", payload))
                }
            }
            "get_time" => {
                let timestamp = chrono::Utc::now().timestamp();
                WebAppResponse {
                    success: true,
                    message: "Current timestamp".to_string(),
                    data:    Some(timestamp.to_string())
                }
            }
            _ => WebAppResponse {
                success: false,
                message: "Unknown action".to_string(),
                data:    None
            }
        };

        // Send response back to WebApp via answerWebAppQuery if available
        // For regular messages, we'll send a regular message
        let response_json = serde_json::to_string(&response).map_err(|e| {
            AppError::new(AppErrorKind::Internal, "Failed to serialize response").with_context(e)
        })?;

        bot.send_message(msg.chat.id, response_json)
            .await
            .map_err(|e| {
                AppError::new(AppErrorKind::Service, "Failed to send response").with_context(e)
            })?;

        tracing::info!(
            "WebApp message from user {}: {}",
            msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
            webapp_msg.action
        );
    }

    Ok(())
}
