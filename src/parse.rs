use std::collections::HashSet;
use url::Url;

use crate::{errors::ParseDataError, models::InitData};

/// Converts passed init data presented as query string to InitData object.
pub fn parse(init_data: &str) -> Result<InitData, ParseDataError> {
    // Parse passed init data as query string
    let url = Url::parse(&format!("http://dummy.com?{}", init_data))?;

    // Properties that should always be interpreted as strings
    let string_props: HashSet<&str> = ["start_param"].iter().cloned().collect();

    // Build JSON map
    let mut map = serde_json::Map::new();
    for (key, value) in url.query_pairs() {
        let val_str = value.into_owned();
        let key_str = key.into_owned();
        if string_props.contains(key_str.as_str()) {
            map.insert(key_str, serde_json::Value::String(val_str));
        } else {
            // Try to parse the value as JSON
            match serde_json::from_str(&val_str) {
                Ok(json_value) => {
                    map.insert(key_str, json_value);
                }
                Err(_) => {
                    // Use as string
                    map.insert(key_str, serde_json::Value::String(val_str));
                }
            }
        }
    }

    // Create final JSON value
    let json_value = serde_json::Value::Object(map);

    // Deserialize JSON into InitData struct
    Ok(serde_json::from_value(json_value)?)
}

#[cfg(test)]
mod tests {

    use crate::{
        models::{InitData, User},
        parse,
    };

    #[test]
    fn test_parse_valid_data() {
        let init_data = "query_id=AAHdF6IQAAAAAN0XohDhrOrc&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%22%2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22%2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%7D&auth_date=1662771648&hash=c501b71e775f74ce10e377dea85a7ea24ecd640b223ea86dfe453e0eaed2e2b2&start_param=abc";
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
        assert!(result.is_err());
    }
}
