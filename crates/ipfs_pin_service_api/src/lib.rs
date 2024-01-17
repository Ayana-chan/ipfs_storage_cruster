// #![warn(missing_docs)]

use async_trait::async_trait;
use axum::{http, Json, Router};
use axum::routing::{get, post, delete};
use crate::models::PinStatus;

pub mod errors;
pub mod models;

pub type ApiResponse<T> = Result<Json<T>, errors::ResponseError>;

/// Should be 'static
#[async_trait]
pub trait IpfsPinServiceApi {
    /// List pin objects
    async fn get_pins(
        Json(get_pins_args): Json<models::GetPinsArgs>,
    ) -> ApiResponse<models::PinResults>;

    /// Add pin object
    async fn add_pin(
        pin: Json<models::Pin>,
    ) -> ApiResponse<models::PinStatus>;

    /// Get pin object
    async fn get_pin_by_request_id(
        requestid: String,
    ) -> ApiResponse<models::PinStatus>;

    /// Replace pin object
    async fn replace_pin_by_request_id(
        requestid: String,
        pin: models::Pin,
    ) -> ApiResponse<models::PinStatus>;

    /// Remove pin object
    async fn delete_pin_by_request_id(
        requestid: String,
    ) -> ApiResponse<()>;
}

pub fn generate_router<T>() -> Router
where T: IpfsPinServiceApi + 'static{
    let ipfs_pin_service_app = Router::new()
        .route("/", get(T::get_pins));
        // .route("/", post(T::add_pin))
        // .route("/:requestid", get(T::get_pin_by_request_id))
        // .route("/:requestid", post(T::replace_pin_by_request_id))
        // .route("/:requestid", delete(T::delete_pin_by_request_id));

    Router::new().nest("/pins", ipfs_pin_service_app)
}

/// convert u16 to http::StatusCode
fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}

#[cfg(test)]
mod tests {
    use crate::models::{GetPinsArgs, Pin, PinResults};
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
    async fn get_test() {
        struct MyApi{}
        #[async_trait]
        impl IpfsPinServiceApi for MyApi {
            async fn get_pins(Json(get_pins_args): Json<GetPinsArgs>) -> ApiResponse<PinResults> {
                panic!("get_pins: {:?}", get_pins_args);
                Ok(Json::from(PinResults::new(0, vec![])))
            }

            async fn add_pin(pin: Json<Pin>) -> ApiResponse<PinStatus> {
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
        }
        let app = generate_router::<MyApi>();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
