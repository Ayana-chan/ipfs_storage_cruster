use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use axum::http::Uri;
use json::JsonValue;
use serde::de::DeserializeOwned;
use super::errors;

/// convert u16 to http::StatusCode
pub fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}

/// an lite error which can convert to std::error::Error
pub struct PureMessageError {
    msg: String,
}

impl PureMessageError {
    pub fn new(msg: &str) -> Self { PureMessageError { msg: msg.to_string() } }
}

impl Debug for PureMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.msg.is_empty() {
            write!(f, "__Empty_Error__")
        } else {
            write!(f, "{}", self.msg)
        }
    }
}

impl Display for PureMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Empty Error")
    }
}

impl std::error::Error for PureMessageError {}

/// Query depend on [serde qs](https://docs.rs/serde_qs/0.12.0/serde_qs).
/// Able to deserialize more complex urlencoded queries, but not omnipotent either.
#[derive(Debug, Clone, Copy, Default)]
pub struct EnhancedQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for EnhancedQuery<T>
    where
        T: DeserializeOwned,
        S: Send + Sync,
{
    type Rejection = errors::ResponseError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::try_from_uri(&parts.uri)
    }
}

impl<T> EnhancedQuery<T>
    where
        T: DeserializeOwned,
{
    /// Get query params from uri, and then parse it into `T`
    pub fn try_from_uri(value: &Uri) -> Result<Self, errors::ResponseError> {
        let query = value.query().unwrap_or_default();
        let params =
            serde_qs::from_str(query)
                .map_err(|e| errors::ResponseError::new(errors::ResponseErrorType::BadRequest)
                    .detail(format!("Query param error: {:?}", e).as_str())
                )?;
        Ok(EnhancedQuery(params))
    }
}

/// A wrapper to help deserialize `map` in query parameter
/// like `http://example.com:3000/search?meta={"admin_name":"123","app_id":"4420da6158cc"}`. \
/// Result is `HashMap<String, String>` (key->value). \
/// `map_parameter` is `Option` to **make it easy to be moved**.
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct MapQueryWrapper {
    /// deserialization result
    pub map_parameter: Option<HashMap<String, String>>,
}

impl MapQueryWrapper {
    /// move out `map_parameter` and destroy self(`MapQueryWrapper`).
    pub fn take_map(self) -> HashMap<String, String> {
        self.map_parameter.unwrap()
    }
}

impl FromStr for MapQueryWrapper {
    type Err = PureMessageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json_value = json::parse(s)
            .map_err(|e| PureMessageError::new(&format!("Fail to parse json: {:?}, because: {:?}", s, e)))?;
        if !json_value.is_object() {
            return Err(PureMessageError::new(&format!("Fail to parse json: {:?}, because: its not an object", s)));
        }

        // convert
        let map_result: HashMap<&str, &JsonValue> = json_value.entries().collect();
        let map_result = map_result.into_iter().map(|entry| (
            entry.0.to_string(), entry.1.to_string()
        )).collect();
        // (|m|
        //     m.into_iter().map(|entry| (
        //         entry.0, entry.1.as_str().unwrap_or_default()
        //     )).collect()
        // );
        Ok(Self {
            map_parameter: Some(map_result),
        })
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use tracing::info;
    use super::*;

    #[test]
    #[ignore]
    fn test_map_from_str() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
        let text = r#"{"admin_name":"123","app_id":"4420da6158cc"}"#;
        let correct_map: HashMap<String, String>= vec![
            ("admin_name".to_string(), "123".to_string()),
            ("app_id".to_string(), "4420da6158cc".to_string())
        ].into_iter().collect();
        let map_ans = MapQueryWrapper::from_str(text).unwrap();
        let map_ans = map_ans.take_map();
        // info!("MapQueryWrapper from: {:?}", text);
        // info!("MapQueryWrapper ans : {:?}", map_ans);
        assert_eq!(map_ans, correct_map);
    }
}

