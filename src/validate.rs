use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use url::Url;

use crate::{errors::ValidationError, sign};

/// Validates the passed init data.
pub fn validate(
    init_data: &str,
    bot_token: &str,
    exp_in: Duration,
) -> Result<bool, ValidationError> {
    // Parse passed init data as query string
    let url = Url::parse(&format!("http://dummy.com?{}", init_data))?;

    let mut auth_date: Option<SystemTime> = None;
    let mut hash: Option<String> = None;
    let mut params: HashMap<String, String> = HashMap::new();

    // Iterate over all key-value pairs of parsed parameters
    for (key, value) in url.query_pairs() {
        let key_str = key.into_owned();
        let value_str = value.into_owned();

        // Store found sign
        if key_str == "hash" {
            hash = Some(value_str.clone());
            continue;
        }
        if key_str == "auth_date" {
            if let Ok(timestamp) = value_str.parse::<u64>() {
                auth_date = Some(UNIX_EPOCH + Duration::from_secs(timestamp));
            }
        }
        // Insert into params
        params.insert(key_str, value_str);
    }

    // Sign is always required
    let hash = hash.ok_or(ValidationError::SignMissing)?;

    // In case expiration time is passed, we do additional parameters check
    if exp_in != Duration::from_secs(0) {
        // In case auth date is none, it means we cannot check if parameters are expired
        let auth_date = auth_date.ok_or(ValidationError::AuthDateMissing)?;

        // Check if init data is expired
        if auth_date + exp_in < SystemTime::now() {
            return Err(ValidationError::Expired);
        }
    }

    // Calculate signature
    let calculated_hash = sign(&params, bot_token, auth_date.unwrap_or(UNIX_EPOCH))
        .map_err(|_| ValidationError::UnexpectedFormat)?;

    // In case our sign is not equal to found one, we should throw an error
    if calculated_hash != hash {
        return Err(ValidationError::SignInvalid);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_validate_success() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22%2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22%2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D&auth_date=1662771648&hash=c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2";
        let bot_token = "5768337691:AAH5YkoiEuPk8-FZa32hStHTqXiLPtAEhx8";
        let exp_in = Duration::from_secs(1662771648);

        assert!(validate(init_data, bot_token, exp_in).is_ok());
    }

    #[test]
    fn test_validate_expired() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22%2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22%2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D&auth_date=1662771648&hash=c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2";
        let bot_token = "your_bot_token";
        let exp_in = Duration::from_secs(1);

        assert!(matches!(
            validate(init_data, bot_token, exp_in),
            Err(ValidationError::Expired)
        ));
    }

    #[test]
    fn test_validate_missing_hash() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22%2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22%2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D&auth_date=1662771648";
        let bot_token = "your_bot_token";
        let exp_in = Duration::from_secs(0);

        assert!(matches!(
            validate(init_data, bot_token, exp_in),
            Err(ValidationError::SignMissing)
        ));
    }

    #[test]
    fn test_validate_invalid_signature() {
        let init_data = "query_id=invalid&user=%7B%22id%22%3A12345%2C%22first_name%22%3A%22Test%22%7D&auth_date=1662771648&hash=invalidhash";
        let bot_token = "5768337691:AAH5YkoiEuPk8-FZa32hStHTqXiLPtAEhx8";
        let exp_in = Duration::from_secs(0);

        assert!(matches!(
            validate(init_data, bot_token, exp_in),
            Err(ValidationError::SignInvalid)
        ));
    }
}
