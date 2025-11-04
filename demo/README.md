# Telegram WebApp SDK Demo

This is a demonstration application showcasing the capabilities of the `telegram-webapp-sdk`.

## Architecture Overview

This demo is a **frontend-only WebApp** that runs inside Telegram. To create a complete Telegram Mini App, you need two components:

1. **WebApp (this demo)** - Frontend application built with Rust/WASM
2. **Bot Backend** - A Telegram bot that receives data from the WebApp

```
┌─────────────────┐         ┌──────────────────┐
│  Telegram User  │         │   Your Server    │
│                 │         │                  │
│  ┌───────────┐  │  HTTP   │  ┌────────────┐  │
│  │  WebApp   │──┼────────▶│  │  Bot API   │  │
│  │  (demo)   │  │  Data   │  │  Handler   │  │
│  └───────────┘  │         │  └────────────┘  │
└─────────────────┘         └──────────────────┘
```

## What This Demo Does

The demo includes several pages:

- **Burger King Demo** (`/burger-king`) - Demonstrates sending data to bot using `send_data()`
- **Init Data** (`/init-data`) - Shows user and chat information from Telegram
- **Launch Parameters** (`/launch-params`) - Displays platform and version info
- **Theme Parameters** (`/theme-params`) - Shows Telegram app color scheme

When a user clicks "Order" in the Burger King demo, the app calls `TelegramWebApp::send_data()` which sends a JSON payload to your bot. **However, you need a bot to receive this data.**

## Building the Demo

### Prerequisites

- Rust nightly toolchain
- `trunk` for building WASM apps:
  ```bash
  cargo install trunk
  ```

### Build

```bash
cd demo
trunk build --release --public-url "https://your-domain.com/path"
```

The built files will be in `demo/dist/`.

### Serve Locally (with mock)

For local development with mock Telegram environment:

```bash
trunk serve
```

Open http://localhost:8080 in your browser.

## Setting Up a Complete Mini App

### Step 1: Deploy the WebApp

1. Build the demo with your public URL
2. Upload `dist/` contents to your web server (must be HTTPS)
3. Your WebApp will be accessible at `https://your-domain.com/path/index.html`

### Step 2: Create a Bot

Create a simple bot to receive data from the WebApp. Here's a Python example:

```python
import json
from telegram import Update
from telegram.ext import Application, CommandHandler, ContextTypes

async def start(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Send WebApp button to user"""
    keyboard = {
        "inline_keyboard": [[
            {
                "text": "Open Burger King",
                "web_app": {"url": "https://your-domain.com/path/index.html#/burger-king"}
            }
        ]]
    }
    await update.message.reply_text(
        "Click the button to open the menu:",
        reply_markup=keyboard
    )

async def handle_web_app_data(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Receive data from WebApp"""
    data = json.loads(update.message.web_app_data.data)

    # Process the order
    item_name = data["name"]
    price = data["price_cents"] / 100

    await update.message.reply_text(
        f"Order received!\n"
        f"Item: {item_name}\n"
        f"Price: ${price:.2f}\n"
        f"Processing your order..."
    )

def main():
    app = Application.builder().token("YOUR_BOT_TOKEN").build()

    app.add_handler(CommandHandler("start", start))
    app.add_handler(MessageHandler(filters.StatusUpdate.WEB_APP_DATA, handle_web_app_data))

    app.run_polling()

if __name__ == "__main__":
    main()
```

### Step 3: Configure Bot in BotFather

1. Open [@BotFather](https://t.me/BotFather)
2. Send `/mybots` and select your bot
3. Choose "Bot Settings" → "Menu Button"
4. Send your WebApp URL: `https://your-domain.com/path/index.html`

### Step 4: Test

1. Open your bot in Telegram
2. Send `/start` (if using the Python example above)
3. Click the WebApp button
4. Select an item and click "Order"
5. The data will be sent to your bot's `handle_web_app_data` function

## Common Issues

### "Clicking buttons doesn't send events to the bot"

This is expected if you haven't set up a bot backend. The WebApp is only the frontend. You need:

1. A bot running on your server (see Python example above)
2. The bot must handle `web_app_data` messages
3. The WebApp must call `send_data()` (which the Burger King demo already does)

### wasm-bindgen version mismatch

If you see:
```
rust Wasm file schema version: 0.2.X
   this binary schema version: 0.2.Y
```

Solution:
```bash
# Update wasm-bindgen-cli to match the version in Cargo.lock
cargo install wasm-bindgen-cli --version $(grep -A1 'name = "wasm-bindgen"' Cargo.lock | grep version | cut -d'"' -f2)
```

Or update dependencies:
```bash
cargo update
trunk build --release
```

### The demo doesn't run in Telegram

Make sure:
- Your WebApp is served over HTTPS (not HTTP)
- The URL is publicly accessible (not localhost)
- You've configured the URL in BotFather

## Project Structure

```
demo/
├── src/
│   ├── main.rs           - Entry point with telegram_app! macro
│   ├── router.rs         - Route definitions
│   ├── pages/            - Page components
│   │   ├── index.rs      - Home page with navigation
│   │   ├── burger_king.rs - Order demo with send_data()
│   │   ├── init_data.rs  - User/chat data display
│   │   ├── launch_params.rs
│   │   └── theme_params.rs
│   └── components/       - Reusable UI components
├── index.html            - HTML template
├── style.css            - Styles
└── Cargo.toml           - Dependencies

```

## Learn More

- [Telegram WebApp Documentation](https://core.telegram.org/bots/webapps)
- [telegram-webapp-sdk Documentation](https://docs.rs/telegram-webapp-sdk)
- [SDK GitHub Repository](https://github.com/RAprogramm/telegram-webapp-sdk)

## License

This demo is part of the `telegram-webapp-sdk` project and is licensed under MIT.
