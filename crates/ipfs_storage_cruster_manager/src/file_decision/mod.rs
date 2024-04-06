use std::fmt::Debug;
use sea_orm::DatabaseConnection;
use axum::async_trait;
use crate::app::common::ApiResult;

pub mod decision_makers;

/// A trait to make decisions to define file storage strategy.
///
/// A maker should be as stateless as possible.
#[async_trait]
pub trait FileStorageDecisionMaker: Send + Sync + Debug {
    /// Decide which nodes to store data on.
    ///
    /// Return target `rpc_address` list.
    async fn decide_store_node(&self,
                         db_conn: &DatabaseConnection,
                         reqwest_client: &reqwest::Client,
    ) -> ApiResult<Vec<String>>;
}

