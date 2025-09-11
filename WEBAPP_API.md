# Telegram WebApp API Coverage

This checklist tracks support for the [Telegram Web Apps JavaScript API](https://core.telegram.org/bots/webapps). Mark items as they are implemented.

## Methods

 - [x] ready
 - [x] expand
 - [x] close
 - [x] sendData
 - [x] openLink
 - [x] openTelegramLink
 - [x] openInvoice
 - [x] readTextFromClipboard
  - [ ] switchInlineQuery
  - [x] showAlert
  - [x] showConfirm
  - [x] showPopup
  - [ ] shareURL
 - [ ] joinVoiceChat
 - [ ] requestWriteAccess
 - [x] requestContact
- [ ] ready
- [ ] expand
- [ ] close
- [ ] sendData
 - [ ] openLink
 - [ ] openTelegramLink
 - [ ] openInvoice
 - [ ] readTextFromClipboard
  - [x] switchInlineQuery
  - [ ] showAlert
  - [ ] showConfirm
  - [ ] showPopup
  - [x] shareURL
- [x] joinVoiceChat
- [x] requestWriteAccess
- [ ] requestContact

## Objects

### MainButton
- [x] show
- [x] hide
- [x] setText
- [x] onClick
- [ ] offClick

### BackButton
- [x] show
- [x] hide
- [x] onClick
- [ ] offClick

### SettingsButton
- [ ] show
- [ ] hide
- [ ] onClick
- [ ] offClick

### HapticFeedback
- [x] impactOccurred
- [x] notificationOccurred
- [x] selectionChanged

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
- [x] Clipboard access
- [ ] Invoice payments
- [ ] Background events
