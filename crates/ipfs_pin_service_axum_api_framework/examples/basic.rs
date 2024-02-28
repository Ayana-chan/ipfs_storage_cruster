#![allow(dead_code)]

use axum::extract::Path;
use tracing::info;
use axum::Json;
use axum::async_trait;
use ipfs_pin_service_axum_api_framework::api::*;
use ipfs_pin_service_axum_api_framework::models::*;
use ipfs_pin_service_axum_api_framework::dto::*;
use ipfs_pin_service_axum_api_framework::EnhancedQuery;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    let app = generate_router::<MyApi>();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug)]
struct MyAuthedUser {
    id: String,
}

struct MyApi {}

#[async_trait]
impl IpfsPinServiceApi for MyApi {
    async fn get_pins(token: AuthContext, EnhancedQuery(get_pins_args): EnhancedQuery<GetPinsArgs>)
                      -> ApiResponse<GetPinsResponse> {
        info!("get_pins args: {:?}", get_pins_args);
        info!("get_pins auth: {:?}", token.token());
        // Err(ResponseError::new(ResponseErrorType::CustomError(477)).detail("miku dayoo"))
        Ok(GetPinsResponse::new(PinResults::new(2, vec![
            PinStatus::new(
                "123456".to_string(),
                Status::Queued,
                Default::default(),
                Pin::new(
                    "ggbbaa".to_string(),
                    None,
                    Some(vec!["www.exp1.com/aaa".to_string(), "www.exp2.com/bbb/ccc".to_string()]),
                    Some(vec![("key1".to_string(), "1".to_string()), ("key2".to_string(), "2".to_string()), ("key3".to_string(), "3".to_string())].into_iter().collect()),
                ),
                vec![],
                None,
            ),
            PinStatus::new(
                "98765".to_string(),
                Status::Pinned,
                Default::default(),
                Pin::new(
                    "ggbbaa".to_string(),
                    None,
                    Some(vec!["www.exp1.com/aaa".to_string(), "www.exp2.com/bbb/ccc".to_string()]),
                    Some(vec![("key1".to_string(), "1".to_string()), ("key2".to_string(), "2".to_string()), ("key3".to_string(), "3".to_string())].into_iter().collect()),
                ),
                vec![],
                None,
            ),
        ])))
    }

    async fn add_pin(token: AuthContext, Json(pin): Json<Pin>) -> ApiResponse<AddPinResponse> {
        info!("add_pin args: {:?}", pin);
        info!("add_pin auth: {:?}", token.token());
        Ok(AddPinResponse::new(PinStatus::new(
            "123456".to_string(),
            Status::Queued,
            Default::default(),
            Pin::new(
                "ggbbaa".to_string(),
                None,
                Some(vec!["www.exp1.com/aaa".to_string(), "www.exp2.com/bbb/ccc".to_string()]),
                Some(vec![("key1".to_string(), "1".to_string()), ("key2".to_string(), "2".to_string()), ("key3".to_string(), "3".to_string())].into_iter().collect()),
            ),
            vec![],
            None,
        )))
    }

    async fn get_pin_by_request_id(token: AuthContext, Path(requestid): Path<String>) -> ApiResponse<GetPinByRequestIdResponse> {
        info!("get_pin_by_request_id args: {:?}", requestid);
        info!("get_pin_by_request_id auth: {:?}", token.token());
        Ok(GetPinByRequestIdResponse::new(PinStatus::new(
            "987654".to_string(),
            Status::Pinned,
            Default::default(),
            Pin::new(
                "ggbbaa".to_string(),
                None,
                Some(vec!["www.exp1.com/aaa".to_string(), "www.exp2.com/bbb/ccc".to_string()]),
                Some(vec![("key1".to_string(), "1".to_string()), ("key2".to_string(), "2".to_string()), ("key3".to_string(), "3".to_string())].into_iter().collect()),
            ),
            vec![],
            None)
        ))
    }

    async fn replace_pin_by_request_id(token: AuthContext, Path(requestid): Path<String>, Json(pin): Json<Pin>) -> ApiResponse<AddPinResponse> {
        info!("replace_pin_by_request_id args: {:?}: {:?}", requestid, pin);
        info!("replace_pin_by_request_id auth: {:?}", token.token());
        Ok(AddPinResponse::new(PinStatus::new(
            "555555".to_string(),
            Status::Pinning,
            Default::default(),
            Pin::new(
                "ggbbaa".to_string(),
                None,
                Some(vec!["www.exp1.com/aaa".to_string(), "www.exp2.com/bbb/ccc".to_string()]),
                Some(vec![("key1".to_string(), "1".to_string()), ("key2".to_string(), "2".to_string()), ("key3".to_string(), "3".to_string())].into_iter().collect()),
            ),
            vec![],
            None)
        ))
    }

    async fn delete_pin_by_request_id(token: AuthContext, Path(requestid): Path<String>) -> ApiResponse<DeletePinByRequestIdResponse> {
        info!("delete_pin_by_request_id args: {:?}", requestid);
        info!("delete_pin_by_request_id auth: {:?}", token.token());
        Ok(DeletePinByRequestIdResponse::new())
    }
}

