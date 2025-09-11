# Telegram WebApp API Coverage

This checklist tracks support for the [Telegram Web Apps JavaScript API](https://core.telegram.org/bots/webapps). Mark items as they are implemented.

## Methods

- [ ] ready
- [ ] expand
- [ ] close
- [ ] sendData
- [ ] openLink
- [ ] openTelegramLink
- [ ] openInvoice
- [ ] switchInlineQuery
- [ ] showAlert
- [ ] showConfirm
- [ ] showPopup
- [ ] shareURL
- [ ] joinVoiceChat
- [ ] requestWriteAccess
- [ ] requestContact

## Objects

### MainButton
- [ ] show
- [ ] hide
- [ ] setText
- [ ] onClick
- [ ] offClick

### BackButton
- [ ] show
- [ ] hide
- [ ] onClick
- [ ] offClick

### SettingsButton
- [ ] show
- [ ] hide
- [ ] onClick
- [ ] offClick

### HapticFeedback
- [ ] impactOccurred
- [ ] notificationOccurred
- [ ] selectionChanged

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
