#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
# SPDX-License-Identifier: MIT

"""
Example Telegram bot that receives data from the Burger King demo WebApp.

Install dependencies:
    pip install python-telegram-bot

Usage:
    python python_bot.py YOUR_BOT_TOKEN https://your-domain.com/path/index.html
"""

import json
import sys
from telegram import Update, WebAppInfo, InlineKeyboardButton, InlineKeyboardMarkup
from telegram.ext import Application, CommandHandler, ContextTypes, MessageHandler, filters


async def start(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """Send WebApp button to user."""
    if len(context.args) < 1:
        webapp_url = context.bot_data.get("webapp_url", "https://example.com")
    else:
        webapp_url = context.args[0]

    keyboard = [
        [
            InlineKeyboardButton(
                "Open Burger King Menu",
                web_app=WebAppInfo(url=f"{webapp_url}#/burger-king")
            )
        ],
        [
            InlineKeyboardButton(
                "View Init Data",
                web_app=WebAppInfo(url=f"{webapp_url}#/init-data")
            )
        ],
    ]

    reply_markup = InlineKeyboardMarkup(keyboard)

    await update.message.reply_text(
        "Welcome to the Telegram WebApp SDK Demo!\n\n"
        "Click a button below to open the WebApp:",
        reply_markup=reply_markup
    )


async def handle_web_app_data(update: Update, context: ContextTypes.DEFAULT_TYPE):
    """
    Handle data sent from the WebApp.

    The demo's Burger King page sends JSON like:
    {"id": 1, "name": "Whopper", "price_cents": 599}
    """
    try:
        data = json.loads(update.message.web_app_data.data)

        item_id = data.get("id")
        item_name = data.get("name", "Unknown")
        price_cents = data.get("price_cents", 0)
        price_dollars = price_cents / 100

        response = (
            f"✅ Order Received!\n\n"
            f"Item: {item_name}\n"
            f"Price: ${price_dollars:.2f}\n"
            f"Order ID: #{item_id}\n\n"
            f"Your order is being processed..."
        )

        await update.message.reply_text(response)

        print(f"[Order] User {update.effective_user.id} ordered: {item_name} (${price_dollars:.2f})")

    except json.JSONDecodeError:
        await update.message.reply_text(
            "❌ Error: Could not parse order data."
        )
    except Exception as e:
        await update.message.reply_text(
            f"❌ Error processing order: {str(e)}"
        )
        print(f"[Error] {e}")


def main():
    if len(sys.argv) < 2:
        print("Usage: python python_bot.py YOUR_BOT_TOKEN [WEBAPP_URL]")
        sys.exit(1)

    bot_token = sys.argv[1]
    webapp_url = sys.argv[2] if len(sys.argv) > 2 else "https://example.com"

    print(f"Starting bot with WebApp URL: {webapp_url}")

    app = Application.builder().token(bot_token).build()
    app.bot_data["webapp_url"] = webapp_url

    app.add_handler(CommandHandler("start", start))
    app.add_handler(MessageHandler(filters.StatusUpdate.WEB_APP_DATA, handle_web_app_data))

    print("Bot is running... Press Ctrl+C to stop.")
    app.run_polling(allowed_updates=Update.ALL_TYPES)


if __name__ == "__main__":
    main()
