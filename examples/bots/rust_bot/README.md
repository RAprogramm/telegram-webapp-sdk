# WebApp Bot Example (Rust)

Example Telegram bot that receives data from the `telegram-webapp-sdk` demo using [teloxide](https://github.com/teloxide/teloxide).

## Features

- Displays WebApp buttons with links to demo pages
- Receives and processes orders from Burger King demo
- Structured logging with tracing
- Environment-based configuration

## Setup

### Prerequisites

- Rust toolchain (1.90+)
- Telegram bot token from [@BotFather](https://t.me/BotFather)
- Deployed WebApp URL (HTTPS required)

### Installation

1. Clone the repository and navigate to this directory:
   ```bash
   cd examples/bots/rust_bot
   ```

2. Copy environment template:
   ```bash
   cp .env.example .env
   ```

3. Edit `.env` and set your values:
   ```env
   TELOXIDE_TOKEN=your_bot_token_from_botfather
   WEBAPP_URL=https://your-domain.com/path/index.html
   RUST_LOG=info
   ```

4. Build and run:
   ```bash
   cargo run --release
   ```

## Usage

1. Open your bot in Telegram
2. Send `/start`
3. Click "Open Burger King Menu"
4. Select items and click "Order"
5. The bot receives the order data and responds

## How It Works

### Sending WebApp Buttons

```rust
let keyboard = InlineKeyboardMarkup::new(vec![
    vec![InlineKeyboardButton::web_app(
        "Open Burger King Menu",
        WebAppInfo {
            url: format!("{}#/burger-king", webapp_url).parse()?,
        },
    )],
]);

bot.send_message(msg.chat.id, "Click to open:")
    .reply_markup(keyboard)
    .await?;
```

### Receiving WebApp Data

```rust
async fn handle_webapp_data(bot: Bot, msg: Message) -> Result<(), teloxide::RequestError> {
    if let Some(web_app_data) = msg.web_app_data() {
        let order: OrderData = serde_json::from_str(&web_app_data.data)?;

        bot.send_message(
            msg.chat.id,
            format!("Order: {} - ${:.2}", order.name, order.price_cents as f64 / 100.0)
        ).await?;
    }
    Ok(())
}
```

## Project Structure

```
rust_bot/
├── src/
│   └── main.rs          # Bot logic with handlers
├── Cargo.toml           # Dependencies
├── .env.example         # Environment template
└── README.md            # This file
```

## Based On

This example is based on [telegram-bot-template](https://github.com/RAprogramm/telegram-bot-template) by RAprogramm.

## License

MIT
