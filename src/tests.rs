#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn test_parse_valid_data() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc\
            &user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22\
            %2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22\
            %2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D\
            &auth_date=1662771648\
            &hash=c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2\
            &start_param=abc";
        let result = parse(init_data);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(
            data,
            InitData {
                auth_date: 1662771648,
                can_send_after: None,
                chat: None,
                chat_type: None,
                chat_instance: None,
                hash: "c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2"
                    .to_string(),
                query_id: Some("AAHdF6IQAAAAAN0XohDhrOrc".to_string()),
                receiver: None,
                start_param: Some("abc".to_string()),
                user: Some(User {
                    added_to_attachment_menu: None,
                    allows_write_to_pm: None,
                    is_premium: Some(true),
                    first_name: "Vladislav".to_string(),
                    id: 279058397,
                    is_bot: None,
                    last_name: Some("Kibenko".to_string()),
                    language_code: Some("ru".to_string()),
                    photo_url: None,
                    username: Some("vdkfrost".to_string())
                })
            }
        );
    }

    #[test]
    fn test_parse_invalid_data() {
        let init_data = "invalid data";
        let result = parse(init_data);
        assert!(matches!(result, Err(ParseDataError::InvalidQueryString(_))));
    }

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

    #[test]
    fn test_validate_success() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc\
            &user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22\
            %2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22\
            %2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D\
            &auth_date=1662771648\
            &hash=c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2";
        let token = "5768337691:AAH5YkoiEuPk8-FZa32hStHTqXiLPtAEhx8";
        let exp_in = Duration::from_secs(1662771648);

        assert!(validate(init_data, token, exp_in).is_ok());
    }

    #[test]
    fn test_validate_expired() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc\
            &user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22\
            %2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22\
            %2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D\
            &auth_date=1662771648\
            &hash=c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2";
        let token = "your_bot_token";
        let exp_in = Duration::from_secs(86400); // 1 day

        assert!(matches!(
            validate(init_data, token, exp_in),
            Err(ValidationError::Expired)
        ));
    }

    #[test]
    fn test_validate_missing_hash() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc\
            &user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22\
            %2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22\
            %2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D\
            &auth_date=1662771648";
        let token = "your_bot_token";
        let exp_in = Duration::from_secs(86400); // 1 day

        assert!(matches!(
            validate(init_data, token, exp_in),
            Err(ValidationError::SignMissing)
        ));
    }

    #[test]
    fn test_validate_invalid_signature() {
        // Get current UNIX timestamp
        let auth_date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Build init_data with the current auth_date
        let init_data = format!(
            "query_id=invalid\
            &user=%7B%22id%22%3A12345%2C%22first_name%22%3A%22Test%22%7D\
            &auth_date={}\
            &hash=invalidhash",
            auth_date
        );

        let bot_token = "5768337691:AAH5YkoiEuPk8-FZa32hStHTqXiLPtAEhx8";
        let exp_in = Duration::from_secs(86400); // 1 day

        assert!(matches!(
            validate(&init_data, bot_token, exp_in),
            Err(ValidationError::SignInvalid)
        ));
    }
}
