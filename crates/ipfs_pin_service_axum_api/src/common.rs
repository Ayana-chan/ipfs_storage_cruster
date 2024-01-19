use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use axum::http::Uri;
use serde::de::DeserializeOwned;
use super::errors;

/// convert u16 to http::StatusCode
pub(crate) fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}

/// Query depend on [serde qs](https://docs.rs/serde_qs/0.12.0/serde_qs).
/// Able to get nested urlencoded queries.
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
    pub fn try_from_uri(value: &Uri) -> Result<Self, errors::ResponseError> {
        let query = value.query().unwrap_or_default();
        let params =
            serde_qs::from_str(query)
                .map_err(|e| errors::ResponseError::new(errors::ResponseErrorType::BadRequest)
                    .detail( format!("Query param error: {:?}", e).as_str() )
                )?;
        Ok(EnhancedQuery(params))
    }
}

