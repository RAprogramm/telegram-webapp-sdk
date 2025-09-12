use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hmac_sha256::{HMAC, Hash};
use percent_encoding::percent_decode_str;
use thiserror::Error;

/// Errors that can occur when validating Telegram init data.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    /// A required field such as `hash` or `signature` was missing.
    #[error("missing required field: {0}")]
    MissingField(&'static str),
    /// Input contained invalid percent encoding or non-UTF8 data.
    #[error("invalid encoding in init data")]
    InvalidEncoding,
    /// Signature value could not be parsed from its encoding (hex or base64).
    #[error("invalid signature encoding")]
    InvalidSignatureEncoding,
    /// Computed signature did not match the provided one.
    #[error("signature mismatch")]
    SignatureMismatch,
    /// Provided Ed25519 public key was malformed.
    #[error("invalid public key")]
    InvalidPublicKey
}

/// Key material used to validate Telegram init data.
#[derive(Clone, Copy, Debug)]
pub enum ValidationKey<'a> {
    /// Validate using a bot token and HMAC-SHA256.
    BotToken(&'a str),
    /// Validate using an Ed25519 public key.
    Ed25519PublicKey(&'a [u8; 32])
}

/// Validates the `hash` parameter of the init data using HMAC-SHA256.
///
/// The `init_data` string must be the exact value of
/// `Telegram.WebApp.initData`. The function derives a secret key from the
/// provided bot token and checks that the `hash` parameter matches the expected
/// HMAC-SHA256.
///
/// # Errors
/// Returns [`ValidationError`] if parsing fails or the hash does not match.
///
/// # Examples
/// ```
/// use hmac_sha256::{HMAC, Hash};
/// use telegram_webapp_sdk::validate_init_data::verify_hmac_sha256;
/// let token = "123456:ABC";
/// let check_string = "auth_date=1\nuser=alice";
/// let secret = Hash::hash(format!("WebAppData{token}").as_bytes());
/// let hash = hex::encode(HMAC::mac(check_string.as_bytes(), secret));
/// let init_data = format!("auth_date=1&user=alice&hash={hash}");
/// assert!(verify_hmac_sha256(&init_data, token).is_ok());
/// ```
pub fn verify_hmac_sha256(init_data: &str, bot_token: &str) -> Result<(), ValidationError> {
    let (check_string, hash) = extract_check_string(init_data, "hash")?;

    let secret_key = Hash::hash(format!("WebAppData{bot_token}").as_bytes());
    let expected = HMAC::mac(check_string.as_bytes(), secret_key);
    let expected_hex = hex::encode(expected);

    if expected_hex == hash {
        Ok(())
    } else {
        Err(ValidationError::SignatureMismatch)
    }
}

/// Validates the `signature` parameter of the init data using Ed25519.
///
/// The `init_data` string must include a `signature` parameter encoded in
/// Base64. All other parameters are combined into the data check string
/// according to Telegram's specification and verified against the provided
/// Ed25519 public key.
///
/// # Errors
/// Returns [`ValidationError`] if parsing fails or the signature does not
/// verify.
///
/// # Examples
/// ```
/// use ed25519_dalek::{Signer, SigningKey};
/// use telegram_webapp_sdk::validate_init_data::verify_ed25519;
///
/// // generate test key
/// let sk = SigningKey::from_bytes(&[1u8; 32]);
/// let pk = sk.verifying_key();
/// let message = "a=1\nb=2";
/// let sig = sk.sign(message.as_bytes());
/// let init_data = format!("a=1&b=2&signature={}", base64::encode(sig.to_bytes()));
/// assert!(verify_ed25519(&init_data, pk.as_bytes()).is_ok());
/// ```
pub fn verify_ed25519(init_data: &str, public_key: &[u8; 32]) -> Result<(), ValidationError> {
    let (check_string, signature_b64) = extract_check_string(init_data, "signature")?;

    let sig_bytes = BASE64_STANDARD
        .decode(signature_b64)
        .map_err(|_| ValidationError::InvalidSignatureEncoding)?;
    let signature = Signature::from_slice(&sig_bytes)
        .map_err(|_| ValidationError::InvalidSignatureEncoding)?;
    let verifying_key =
        VerifyingKey::from_bytes(public_key).map_err(|_| ValidationError::InvalidPublicKey)?;

    verifying_key
        .verify(check_string.as_bytes(), &signature)
        .map_err(|_| ValidationError::SignatureMismatch)
}

fn extract_check_string(
    init_data: &str,
    signature_field: &'static str
) -> Result<(String, String), ValidationError> {
    let mut data: Vec<(String, String)> = Vec::new();
    let mut signature: Option<String> = None;

    for pair in init_data.split('&') {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().ok_or(ValidationError::InvalidEncoding)?;
        let value = parts.next().ok_or(ValidationError::InvalidEncoding)?;
        let decoded = percent_decode_str(value)
            .decode_utf8()
            .map_err(|_| ValidationError::InvalidEncoding)?
            .to_string();
        if key == signature_field {
            signature = Some(decoded);
        } else {
            data.push((key.to_string(), decoded));
        }
    }

    let signature = signature.ok_or(ValidationError::MissingField(signature_field))?;

    data.sort_by(|a, b| a.0.cmp(&b.0));
    let check_string = data
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");

    Ok((check_string, signature))
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Signer, SigningKey};

    use super::*;

    #[test]
    fn hmac_validates() {
        let bot_token = "123456:ABC";
        let secret_key = Hash::hash(format!("WebAppData{bot_token}").as_bytes());
        let check_string = "a=1\nb=2";
        let expected = HMAC::mac(check_string.as_bytes(), secret_key);
        let hash = hex::encode(expected);
        let query = format!("a=1&b=2&hash={hash}");
        assert!(verify_hmac_sha256(&query, bot_token).is_ok());
    }

    #[test]
    fn hmac_rejects_modified_data() {
        let bot_token = "123456:ABC";
        let secret_key = Hash::hash(format!("WebAppData{bot_token}").as_bytes());
        let check_string = "a=1\nb=2";
        let expected = HMAC::mac(check_string.as_bytes(), secret_key);
        let hash = hex::encode(expected);
        // tamper with data
        assert_eq!(
            verify_hmac_sha256(&format!("a=1&b=3&hash={hash}"), bot_token),
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
        assert!(verify_ed25519(&init_data, pk.as_bytes()).is_ok());
    }

    #[test]
    fn ed25519_rejects_bad_signature() {
        let sk = SigningKey::from_bytes(&[42u8; 32]);
        let pk = sk.verifying_key();
        let message = "a=1\nb=2";
        let sig = sk.sign(message.as_bytes());
        // modify data
        let tampered = format!(
            "a=1&b=3&signature={}",
            BASE64_STANDARD.encode(sig.to_bytes())
        );
        assert_eq!(
            verify_ed25519(&tampered, pk.as_bytes()),
            Err(ValidationError::SignatureMismatch)
        );
    }
}
