use hex::encode;
use hmac_sha256::{HMAC, Hash};
use percent_encoding::percent_decode_str;

/// Verifies the `hash` of Telegram init data using the secret key derived from
/// the bot token.
///
/// # Arguments
/// - `init_data`: raw query string from `Telegram.WebApp.initData`, e.g.
///   `"user=%7B%22id%22%3A12345...%7D&auth_date=...&hash=..."`
/// - `bot_token`: your bot token, e.g.
///   `"123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"`
///
/// # Returns
/// - `true` if the hash is valid and matches Telegram's computation rules
/// - `false` otherwise
pub fn verify_init_data_hash(init_data: &str, bot_token: &str) -> bool {
    let parsed = match parse_init_data(init_data) {
        Some(pairs) => pairs,
        None => return false
    };

    let mut actual_hash: Option<String> = None;
    let mut data: Vec<(String, String)> = Vec::new();

    for (k, v) in parsed {
        if k == "hash" {
            actual_hash = Some(v);
        } else {
            data.push((k, v));
        }
    }

    let actual_hash = match actual_hash {
        Some(h) => h,
        None => return false
    };

    data.sort_by(|a, b| a.0.cmp(&b.0));

    let check_string = data
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");

    let secret_key = Hash::hash(format!("WebAppData{}", bot_token).as_bytes());
    let expected_hash = HMAC::mac(check_string.as_bytes(), secret_key);
    let expected_hex = encode(expected_hash);

    expected_hex == actual_hash
}

/// Parses the raw `init_data` query string into key-value pairs.
///
/// # Returns
/// - `Some(Vec<(key, value)>)` if successfully parsed
/// - `None` on any decoding or structural error
fn parse_init_data(init_data: &str) -> Option<Vec<(String, String)>> {
    let mut result = Vec::new();

    for pair in init_data.split('&') {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next()?.to_string();
        let val = parts.next()?;
        let decoded_val = percent_decode_str(val).decode_utf8().ok()?.to_string();
        result.push((key, decoded_val));
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_format_fails() {
        let input = "this_is_not_query_string";
        assert!(!verify_init_data_hash(input, "123:token"));
    }
}
