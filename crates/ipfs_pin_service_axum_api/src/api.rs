use std::fmt::Debug;
use axum::{extract, http, Json, middleware, response, Router};
use axum::routing::{get, post, delete};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use crate::errors::{ResponseError, ResponseErrorType};
use crate::models;

/// The **key** of access-token in request header
pub const AUTH_KEY: &str = "Authorization";

/// A type wrap for API's response
pub type ApiResponse<T> = Result<Json<T>, ResponseError>;

/// `impl IpfsPinServiceApi for your_struct` to create an API implementation. \
/// AuthenticatedUser: **User information** type, which is obtained through **auth token**.
/// Should be `'static`.
#[async_trait]
pub trait IpfsPinServiceApi<A> {
    /// List pin objects.
    async fn get_pins(
        auth_context: AuthContext<A>,
        Json(get_pins_args): Json<models::GetPinsArgs>,
    ) -> ApiResponse<models::PinResults>;

    /// Add pin object.
    async fn add_pin(
        Json(pin): Json<models::Pin>,
    ) -> ApiResponse<models::PinStatus>;

    /// Get pin object.
    async fn get_pin_by_request_id(
        requestid: String,
    ) -> ApiResponse<models::PinStatus>;

    /// Replace pin object. \
    /// This is a shortcut for removing a pin object identified by requestid
    /// and creating a new one in a single API call
    /// that protects against undesired garbage collection of blocks common to both pins.
    /// Useful when updating a pin representing a huge dataset where most of blocks did not change.
    /// The new pin object requestid is returned in the PinStatus response.
    /// The old pin object is deleted automatically.
    async fn replace_pin_by_request_id(
        requestid: String,
        pin: models::Pin,
    ) -> ApiResponse<models::PinStatus>;

    /// Remove pin object.
    async fn delete_pin_by_request_id(
        requestid: String,
    ) -> ApiResponse<()>;

    /// Function to verify the token and get corresponding user information
    async fn verify_token(token: &str) -> Result<A, ()>;
}

/// Generate axum router by type impl `IpfsPinServiceApi`.
pub fn generate_router<T, A>() -> Router
    where T: IpfsPinServiceApi<A> + 'static {
    let ipfs_pin_service_app = Router::new()
        .route("/", get(T::get_pins));
    // .route("/", post(T::add_pin))
    // .route("/:requestid", get(T::get_pin_by_request_id))
    // .route("/:requestid", post(T::replace_pin_by_request_id))
    // .route("/:requestid", delete(T::delete_pin_by_request_id));

    // add pins prefix
    let ipfs_pin_service_app = Router::new().nest("/pins", ipfs_pin_service_app);
    // add global 404 handler
    let ipfs_pin_service_app = ipfs_pin_service_app.fallback(handle_404);

    ipfs_pin_service_app
}

/// Intermediary of auth
#[derive(Debug)]
pub struct AuthContext<A>
    where A: Debug {
    token: String,
    authed_user: A,
}

impl<A> AuthContext<A>
    where A: Debug {
    pub fn new(token: &str, authed_user: A) -> Self {
        Self {
            token: token.to_string(),
            authed_user,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn authed_user(&self) -> &A {
        &self.authed_user
    }
}

#[async_trait]
impl<S, T, A> FromRequestParts<S> for AuthContext<A>
    where S: Send + Sync,
          T: IpfsPinServiceApi<A>,
          A: Debug {
    type Rejection = response::Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = http::HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|err| match err {})?;

        // get token from http header
        let token = headers.get(AUTH_KEY);
        if token.is_none() {
            return Err(ResponseError::new(ResponseErrorType::Unauthorized).into_response());
        }
        let token = token.unwrap().to_str();
        if token.is_err() {
            return Err(ResponseError::new(ResponseErrorType::Unauthorized).into_response());
        }
        let token = token.unwrap();

        // verify token
        let authed_user = T::verify_token(token).await;
        if authed_user.is_err() {
            // verify failed
            return Err(ResponseError::new(ResponseErrorType::Unauthorized).into_response());
        }
        let authed_user = authed_user.unwrap();

        Ok(AuthContext::new(token, authed_user))
    }
}

// async fn auth<T>(
//     headers: http::HeaderMap,
//     request: extract::Request,
//     next: middleware::Next,
// ) -> Result<response::Response, ResponseError>
//     where T: IpfsPinServiceApi + 'static {
//     match get_token(&headers) {
//         Some(token) if T::verify_token(token).await => {
//             let response = next.run(request).await;
//             Ok(response)
//         }
//         _ => {
//             Err(ResponseError::new(ResponseErrorType::Unauthorized))
//         }
//     }
// }
//
// fn get_token(headers: &http::HeaderMap) -> Option<&str> {
//     None
// }

async fn handle_404() -> ResponseError {
    ResponseError::new(ResponseErrorType::NotFound)
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use super::*;

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

    #[tokio::test]
    #[ignore]
    #[allow(warnings)]
    async fn test_basic() {
        struct MyApi {}
        #[derive(Debug)]
        struct MyAuthedUser {
            id: String,
        }
        #[async_trait]
        impl<A> IpfsPinServiceApi<A> for MyApi {
            async fn get_pins(auth_context: AuthContext<A>, Json(get_pins_args): Json<GetPinsArgs>) -> ApiResponse<PinResults> {
                println!("get_pins: {:?}", get_pins_args);
                println!("get_pins auth: {:?}", auth_context.authed_user());
                Ok(Json::from(PinResults::new(0, vec![])))
            }

            async fn add_pin(Json(pin): Json<Pin>) -> ApiResponse<PinStatus> {
                todo!()
            }

            async fn get_pin_by_request_id(requestid: String) -> ApiResponse<PinStatus> {
                todo!()
            }

            async fn replace_pin_by_request_id(requestid: String, pin: Pin) -> ApiResponse<PinStatus> {
                todo!()
            }

            async fn delete_pin_by_request_id(requestid: String) -> ApiResponse<()> {
                todo!()
            }

            async fn verify_token(token: &str) -> Result<A, ()> {
                println!("verify_token: {}", token);
                Ok(Self::AuthenticatedUser {
                    id: "miku114514".to_string()
                })
            }
        }
        let app = generate_router::<MyApi, MyAuthedUser>();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

