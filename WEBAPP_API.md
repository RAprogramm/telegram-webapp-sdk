# Telegram WebApp API Coverage

This checklist tracks support for the [Telegram Web Apps JavaScript API](https://core.telegram.org/bots/webapps). Mark items as
they are implemented.

## Methods

- [x] ready ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] expand ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] close ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] sendData ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] openLink ([840ace1](https://github.com/RAprogramm/telegram-webapp-sdk/commit/840ace1))
- [x] openTelegramLink ([840ace1](https://github.com/RAprogramm/telegram-webapp-sdk/commit/840ace1))
- [x] openInvoice ([840ace1](https://github.com/RAprogramm/telegram-webapp-sdk/commit/840ace1))
- [x] switchInlineQuery ([a098e00](https://github.com/RAprogramm/telegram-webapp-sdk/commit/a098e00))
- [x] showAlert ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] showConfirm ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] showPopup ([840ace1](https://github.com/RAprogramm/telegram-webapp-sdk/commit/840ace1))
- [x] showScanQrPopup ([840ace1](https://github.com/RAprogramm/telegram-webapp-sdk/commit/840ace1))
- [x] closeScanQrPopup ([840ace1](https://github.com/RAprogramm/telegram-webapp-sdk/commit/840ace1))
- [x] shareURL ([a098e00](https://github.com/RAprogramm/telegram-webapp-sdk/commit/a098e00))
- [x] joinVoiceChat ([a098e00](https://github.com/RAprogramm/telegram-webapp-sdk/commit/a098e00))
- [x] requestWriteAccess ([a098e00](https://github.com/RAprogramm/telegram-webapp-sdk/commit/a098e00))
- [x] requestContact ([d595540](https://github.com/RAprogramm/telegram-webapp-sdk/commit/d595540))
- [x] requestPhoneNumber ([d595540](https://github.com/RAprogramm/telegram-webapp-sdk/commit/d595540))
- [x] openContact ([d595540](https://github.com/RAprogramm/telegram-webapp-sdk/commit/d595540))

## Objects

### MainButton
- [x] show ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] hide ([f0a108d](https://github.com/RAprogramm/telegram-webapp-sdk/commit/f0a108d))
- [x] setText ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] onClick ([0a42d7b](https://github.com/RAprogramm/telegram-webapp-sdk/commit/0a42d7b))
- [x] offClick ([0a42d7b](https://github.com/RAprogramm/telegram-webapp-sdk/commit/0a42d7b))

### BackButton
- [x] show ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] hide ([bcce132](https://github.com/RAprogramm/telegram-webapp-sdk/commit/bcce132))
- [x] onClick ([0a42d7b](https://github.com/RAprogramm/telegram-webapp-sdk/commit/0a42d7b))
- [x] offClick ([0a42d7b](https://github.com/RAprogramm/telegram-webapp-sdk/commit/0a42d7b))

### SettingsButton
- [ ] show
- [ ] hide
- [ ] onClick
- [ ] offClick

### HapticFeedback
- [x] impactOccurred ([9896d92](https://github.com/RAprogramm/telegram-webapp-sdk/commit/9896d92))
- [x] notificationOccurred ([9896d92](https://github.com/RAprogramm/telegram-webapp-sdk/commit/9896d92))
- [x] selectionChanged ([9896d92](https://github.com/RAprogramm/telegram-webapp-sdk/commit/9896d92))

### BiometricManager
- [ ] isBiometricAvailable
- [ ] authenticate

### CloudStorage
- [ ] getItem
- [ ] setItem
- [ ] removeItem
- [ ] getItems
- [ ] setItems
- [ ] removeItems
- [ ] getKeys
- [ ] clear

## Remaining WebApp Features

The following features are not yet covered by the SDK:

- [ ] Init data validation
- [ ] Theme change events
- [ ] Viewport management
- [ ] Location access
- [ ] Clipboard access
- [ ] Invoice payments
- [ ] Background events
