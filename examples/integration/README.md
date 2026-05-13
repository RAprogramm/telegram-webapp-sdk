# Telegram WebApp SDK Integration Example

This example demonstrates how to integrate `telegram-webapp-sdk` (frontend) with `teloxide` (backend) to create a full-stack Telegram Mini App.

## Architecture

```
Telegram Mini App (WASM) ←→ Bot Server (Rust)
telegram-webapp-sdk          teloxide
```

## Components

### Frontend (`examples/integration/frontend`)
- Pure WebAssembly using `telegram-webapp-sdk`
- Demonstrates sending data to bot via `TelegramWebApp::sendData()`
- Shows how to receive and display responses

### Backend (`examples/integration/backend`)
- Teloxide-based Telegram bot
- Receives WebApp data via `message.web_app_data()`
- Processes requests and sends responses back

## How It Works

1. User clicks button in Telegram to open Mini App
2. Mini App loads and initializes `telegram-webapp-sdk`
3. User interacts with UI (e.g., types text and clicks "Echo")
4. Mini App sends JSON message to bot using `TelegramWebApp::sendData()`
5. Bot receives update with `web_app_data` field
6. Bot parses JSON, processes request, creates response
7. Bot sends response message back to user
8. Mini App could display response (in this demo, we simulate it)

## Running the Example

### Backend (Bot Server)

```bash
cd examples/integration/backend
cargo run
```

Set environment variables:
- `BOT_TOKEN`: Your Telegram bot token from @BotFather
- `WEBAPP_URL`: URL where frontend will be served (for keyboard buttons)

### Frontend (Mini App)

```bash
cd examples/integration/frontend
trunk serve
```

Then open the displayed URL in Telegram WebApp environment.

## Notes

- This is a simplified demo - in production you'd use `answerWebAppQuery` for direct responses
- The frontend simulates server communication for clarity
- Real integration would use proper async messaging
- See `examples/bots/rust_bot/` for a more complete webhook example

## Files

- `backend/src/main.rs` - Bot logic with teloxide
- `frontend/src/lib.rs` - WebApp logic with telegram-webapp-sdk
- `frontend/index.html` - Simple UI
- `frontend/Trunk.toml` - Trunk WASM build configuration