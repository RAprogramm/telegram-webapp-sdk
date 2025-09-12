use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use ed25519_dalek::{Signer, SigningKey};
use hmac_sha256::{HMAC, Hash};
use telegram_webapp_sdk::{
    TelegramWebApp,
    validate_init_data::{ValidationError, ValidationKey}
};

#[test]
fn hmac_validates() {
    let bot_token = "123456:ABC";
    let secret_key = Hash::hash(format!("WebAppData{bot_token}").as_bytes());
    let check_string = "a=1\nb=2";
    let expected = HMAC::mac(check_string.as_bytes(), secret_key);
    let hash = hex::encode(expected);
    let query = format!("a=1&b=2&hash={hash}");
    assert!(
        TelegramWebApp::validate_init_data(&query, ValidationKey::BotToken(bot_token)).is_ok()
    );
}

#[test]
fn hmac_rejects_modified_data() {
    let bot_token = "123456:ABC";
    let secret_key = Hash::hash(format!("WebAppData{bot_token}").as_bytes());
    let check_string = "a=1\nb=2";
    let expected = HMAC::mac(check_string.as_bytes(), secret_key);
    let hash = hex::encode(expected);
    assert_eq!(
        TelegramWebApp::validate_init_data(
            &format!("a=1&b=3&hash={hash}"),
            ValidationKey::BotToken(bot_token)
        ),
        Err(ValidationError::SignatureMismatch)
    );
}

#[test]
fn ed25519_validates() {
    let sk = SigningKey::from_bytes(&[42u8; 32]);
    let pk = sk.verifying_key();
    let message = "a=1\nb=2";
    let sig = sk.sign(message.as_bytes());
    let init_data = format!(
        "a=1&b=2&signature={}",
        BASE64_STANDARD.encode(sig.to_bytes())
    );
    assert!(
        TelegramWebApp::validate_init_data(
            &init_data,
            ValidationKey::Ed25519PublicKey(pk.as_bytes())
        )
        .is_ok()
    );
}

#[test]
fn ed25519_rejects_bad_signature() {
    let sk = SigningKey::from_bytes(&[42u8; 32]);
    let pk = sk.verifying_key();
    let message = "a=1\nb=2";
    let sig = sk.sign(message.as_bytes());
    let tampered = format!(
        "a=1&b=3&signature={}",
        BASE64_STANDARD.encode(sig.to_bytes())
    );
    assert_eq!(
        TelegramWebApp::validate_init_data(
            &tampered,
            ValidationKey::Ed25519PublicKey(pk.as_bytes())
        ),
        Err(ValidationError::SignatureMismatch)
    );
}
