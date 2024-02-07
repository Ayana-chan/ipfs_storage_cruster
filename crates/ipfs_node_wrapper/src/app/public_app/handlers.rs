use axum::extract::{Query, State};
use tracing::info;
use crate::app::public_app::PublicAppState;
use crate::models;
use crate::ipfs_client;

#[tracing::instrument]
pub async fn get_file(State(state): State<PublicAppState>, Query(query): Query<models::GetFileArgs>) -> Result<String, String> {
    info!("Get File");
    let content = ipfs_client::ipfs_get_file(&query.cid, &state.app_state.ipfs_node_metadata).await?;
    Ok(content)
}


