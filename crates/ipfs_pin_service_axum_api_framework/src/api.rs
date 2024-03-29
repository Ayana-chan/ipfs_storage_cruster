use std::fmt::Debug;
use axum::{http, Json, response, Router};
use axum::routing::{get, post, delete};
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::async_trait;
use crate::errors::{ResponseError, ResponseErrorType};
use crate::{EnhancedQuery, models, dto};

/// The **key** of access-token in request header
pub const AUTH_KEY: &str = "Authorization";

/// A type wrap for API's response
pub type ApiResponse<T> = Result<T, ResponseError>;

/// `impl IpfsPinServiceApi for your_struct` to create an API implementation. \
/// AuthenticatedUser: **User information** type, which is obtained through **auth token**.
/// Should be `'static`.
#[async_trait]
pub trait IpfsPinServiceApi {
    /// List pin objects.
    async fn get_pins(
        token: AuthContext,
        EnhancedQuery(get_pins_args): EnhancedQuery<dto::GetPinsArgs>,
    ) -> ApiResponse<dto::GetPinsResponse>;

    /// Add pin object.
    async fn add_pin(
        token: AuthContext,
        Json(pin): Json<models::Pin>,
    ) -> ApiResponse<dto::AddPinResponse>;

    /// Get pin object.
    async fn get_pin_by_request_id(
        token: AuthContext,
        Path(requestid): Path<String>,
    ) -> ApiResponse<dto::GetPinByRequestIdResponse>;

    /// Replace pin object. \
    /// This is a shortcut for removing a pin object identified by requestid
    /// and creating a new one in a single API call
    /// that protects against undesired garbage collection of blocks common to both pins.
    /// Useful when updating a pin representing a huge dataset where most of blocks did not change.
    /// The new pin object requestid is returned in the PinStatus response.
    /// The old pin object is deleted automatically. \
    /// **NOTE**: **Replace pin** and **Add pin** are basically equivalent in response to business needs,
    /// so `replace_pin_by_request_id` returns `vo::AddPinResponse`
    async fn replace_pin_by_request_id(
        token: AuthContext,
        Path(requestid): Path<String>,
        Json(pin): Json<models::Pin>,
    ) -> ApiResponse<dto::AddPinResponse>;

    /// Remove pin object.
    async fn delete_pin_by_request_id(
        token: AuthContext,
        Path(requestid): Path<String>,
    ) -> ApiResponse<dto::DeletePinByRequestIdResponse>;
}

/// Generate axum router by type impl `IpfsPinServiceApi`.
pub fn generate_router<T>() -> Router
    where T: IpfsPinServiceApi + 'static {
    let ipfs_pin_service_app = Router::new()
        .route("/", get(T::get_pins))
        .route("/", post(T::add_pin))
        .route("/:requestid", get(T::get_pin_by_request_id))
        .route("/:requestid", post(T::replace_pin_by_request_id))
        .route("/:requestid", delete(T::delete_pin_by_request_id));

    // add pins prefix
    let ipfs_pin_service_app = Router::new().nest("/pins", ipfs_pin_service_app);
    // add global 404 handler
    let ipfs_pin_service_app = ipfs_pin_service_app.fallback(handle_404);

    ipfs_pin_service_app
}

/// Intermediary of auth
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct AuthContext {
    pub token: String,
}

impl AuthContext {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

// TODO async_trait is not free
#[async_trait]
impl<S> FromRequestParts<S> for AuthContext
    where S: Send + Sync {
    type Rejection = response::Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = http::HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|err| match err {})?;

        // get token from http header
        let token = headers.get(AUTH_KEY);
        if token.is_none() {
            // here to decide return empty token or refuse request
            return Ok(AuthContext::new(""));
            // return Err(ResponseError::new(ResponseErrorType::Unauthorized).into_response());
        }
        let token = token.unwrap().to_str();
        if token.is_err() {
            return Err(ResponseError::new(ResponseErrorType::Unauthorized).into_response());
        }
        let token = token.unwrap();

        Ok(AuthContext::new(token))
    }
}

async fn handle_404() -> ResponseError {
    ResponseError::new(ResponseErrorType::NotFound)
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn try_async_trait() {
        #[async_trait]
        trait MyAsyncTrait {
            fn sample_func() -> String;
        }
        struct MyStruct {}
        impl MyAsyncTrait for MyStruct {
            fn sample_func() -> String {
                "hello async trait!".to_string()
            }
        }
        let va = MyStruct::sample_func();
        println!("{}", va);
    }
}

