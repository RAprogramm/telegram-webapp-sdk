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
 - [x] ready
 - [x] expand
 - [x] close
 - [x] hideKeyboard
 - [x] sendData
 - [x] openLink
 - [x] openTelegramLink
 - [x] openInvoice
 - [x] downloadFile
 - [ ] switchInlineQuery
 - [x] switchInlineQuery
 - [x] showAlert
 - [x] showConfirm
 - [x] showPopup
 - [x] shareURL
 - [x] shareMessage
 - [x] shareToStory
 - [x] joinVoiceChat
 - [x] requestWriteAccess
 - [x] requestContact
- [ ] shareURL
- [ ] joinVoiceChat
- [x] requestWriteAccess
- [x] requestContact
 - [ ] shareURL
 - [x] readTextFromClipboard
  - [ ] switchInlineQuery
  - [x] showAlert
  - [x] showConfirm
  - [x] showPopup
  - [ ] shareURL
 - [ ] joinVoiceChat
 - [ ] requestWriteAccess
 - [x] setEmojiStatus
 - [x] requestEmojiStatusAccess
 - [x] requestContact
 - [x] setHeaderColor
 - [x] setBackgroundColor
 - [x] setBottomBarColor
- [ ] ready
- [ ] expand
- [ ] close
- [ ] hideKeyboard
- [ ] sendData
- [ ] openLink
- [ ] openTelegramLink
 - [ ] openInvoice
 - [ ] downloadFile
 - [x] switchInlineQuery
- [ ] showAlert
- [ ] showConfirm
- [ ] showPopup
- [x] shareURL
- [x] joinVoiceChat
- [x] requestWriteAccess
 - [x] setEmojiStatus
 - [x] requestEmojiStatusAccess
- [ ] requestContact

## Objects

### BottomButton (Main & Secondary)
- [x] show
- [x] hide
- [x] setText
- [x] setColor
- [x] setTextColor
- [x] onClick
- [ ] offClick
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
- [x] show
- [x] hide
- [x] onClick
- [x] offClick

### HapticFeedback
- [x] impactOccurred ([9896d92](https://github.com/RAprogramm/telegram-webapp-sdk/commit/9896d92))
- [x] notificationOccurred ([9896d92](https://github.com/RAprogramm/telegram-webapp-sdk/commit/9896d92))
- [x] selectionChanged ([9896d92](https://github.com/RAprogramm/telegram-webapp-sdk/commit/9896d92))

### Accelerometer
- [x] start
- [x] stop
- [x] getAcceleration

### Gyroscope
- [x] start
- [x] stop
- [x] getAngularVelocity

### DeviceOrientation
- [x] start
- [x] stop
- [x] getOrientation

### BiometricManager
- [ ] isBiometricAvailable
- [ ] authenticate

### CloudStorage
- [x] getItem
- [x] setItem
- [x] removeItem
- [x] getItems
- [ ] setItems
- [x] removeItems
- [x] getKeys
- [x] clear

## Remaining WebApp Features

The following features are not yet covered by the SDK:

- [ ] Init data validation
- [x] Theme and safe area change events
- [ ] Viewport management
- [ ] Clipboard access
- [ ] Location access
- [x] Clipboard access
- [ ] Invoice payments
- [ ] Background events
