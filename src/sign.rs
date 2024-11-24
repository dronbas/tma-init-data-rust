use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::errors::SignError;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

type HmacSha256 = Hmac<Sha256>;

/// Signs the payload using the specified key.
pub fn sign(
    payload: &HashMap<String, String>,
    bot_token: &str,
    auth_time: SystemTime,
) -> Result<String, SignError> {
    let auth_date = auth_time.duration_since(UNIX_EPOCH)?;

    // Collect and filter pairs
    let mut pairs: Vec<(String, String)> = payload
        .iter()
        .filter(|&(k, _)| k != "hash" && k != "auth_date")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    // Add the auth_date
    pairs.push(("auth_date".to_string(), auth_date.as_secs().to_string()));

    // Sort pairs by key
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Build the payload
    let payload_string = pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("\n");

    // First HMAC: Create secret key using "WebAppData"
    let mut sk_hmac = HmacSha256::new_from_slice(b"WebAppData")
        .map_err(|_| SignError::CouldNotProcessSignature)?;
    sk_hmac.update(bot_token.as_bytes());
    let secret_key = sk_hmac.finalize().into_bytes();

    // Second HMAC: Sign the payload using the secret key
    let mut imp_hmac =
        HmacSha256::new_from_slice(&secret_key).map_err(|_| SignError::CouldNotProcessSignature)?;
    imp_hmac.update(payload_string.as_bytes());

    // Get result and convert to hex string
    let result = imp_hmac.finalize().into_bytes();

    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_sign() {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert(
            "query_id".to_string(),
            "AAHdF6IQAAAAAN0XohDhrOrc".to_string(),
        );
        payload.insert(
            "user".to_string(),
            r#"{"id":279058397,"first_name":"Vladislav","last_name":"Kibenko","username":"vdkfrost","language_code":"ru","is_premium":true}"#.to_string(),
        );
        let bot_token = "5768337691:AAH5YkoiEuPk8-FZa32hStHTqXiLPtAEhx8";
        let auth_time = UNIX_EPOCH + Duration::from_secs(1662771648);

        let expected_hash =
            "c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2".to_string();
        let result = sign(&payload, bot_token, auth_time).unwrap();
        assert_eq!(result, expected_hash);
    }
}
