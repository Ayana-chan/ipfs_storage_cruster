use axum::extract::Query;
use tracing::info;
use crate::models;
use crate::ipfs_client;

#[tracing::instrument]
pub async fn get_file(Query(query): Query<models::GetFileArgs>) -> Result<String, String> {
    info!("Get File");
    let content = ipfs_client::ipfs_get_file(&query.cid).await?;
    Ok(content)
}


