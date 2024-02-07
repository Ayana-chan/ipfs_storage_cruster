use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use tracing::{info, trace};
use crate::app::public_app::PublicAppState;
use crate::models;
use crate::ipfs_client;

#[axum_macros::debug_handler]
#[tracing::instrument(skip_all)]
pub async fn get_file(State(state): State<PublicAppState>, Query(query): Query<models::GetFileArgs>) -> Result<(HeaderMap, impl IntoResponse), String> {
    info!("Get File");
    let ipfs_res = ipfs_client::ipfs_get_file(&query.cid, &state.app_state.ipfs_node_metadata).await;
    if let Err(e) = ipfs_res {
        return Err(e); //TODO 定制error
    }
    let ipfs_res = ipfs_res.unwrap();

    // construct header
    let ipfs_res_header = ipfs_res.headers();
    trace!("header: {:#?}", ipfs_res_header);
    let mut header = HeaderMap::new();
    let header_value;
    header_value = ipfs_res_header
        .get("content-type")
        .map(|v|
            v.to_str().unwrap_or_default()
        );
    if let Some(v) = header_value {
        let hv = HeaderValue::from_str(v);
        if let Ok(hv) = hv {
            header.insert("content-type", hv);
        }
    }

    let content = ipfs_res.text().await;
    if let Err(e) = content {
        return Err(e.to_string());
    }
    let content = content.unwrap();
    trace!("text: {:?}", content);

    Ok((header.to_owned(), content))
}


