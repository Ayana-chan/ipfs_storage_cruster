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
    /// Return target node list.
    async fn decide_store_node(&self,
                               db_conn: &DatabaseConnection,
                               reqwest_client: &reqwest::Client,
    ) -> ApiResult<Vec<TargetIPFSNodeMessage>>;

    /// Decide which nodes to re-store data on when a store failure occurs.
    ///
    /// Return `errors::IPFS_NODE_CLUSTER_UNHEALTHY` to stop store file.
    async fn decide_store_node_fail_one(&self,
                                        db_conn: &DatabaseConnection,
                                        reqwest_client: &reqwest::Client,
    ) -> ApiResult<Vec<TargetIPFSNodeMessage>>;
}

#[derive(Clone, Debug)]
/// Message about IPFS node to contact.
pub struct TargetIPFSNodeMessage {
    /// Node's id in database.
    pub id: String,
    /// RPC address to contact.
    pub rpc_address: String,
}
