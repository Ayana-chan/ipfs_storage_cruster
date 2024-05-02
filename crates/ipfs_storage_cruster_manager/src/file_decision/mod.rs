use std::fmt::Debug;
use sea_orm::DatabaseConnection;
use axum::async_trait;
use crate::app::common::ApiResult;
use crate::imports::dao_imports::*;

pub mod decision_makers;

/// A trait to make decisions to define file storage strategy.
///
/// A maker should be as stateless as possible.
#[async_trait]
pub trait FileStorageDecisionMaker: Send + Sync + Debug {
    /// Decide which nodes to store data on.
    ///
    /// Return target node list.
    /// Returning an empty vec would cause an error (`IPFS_NODE_CLUSTER_UNHEALTHY`).
    async fn decide_store_node(&self,
                               cid: &str,
                               db_conn: &DatabaseConnection,
                               reqwest_client: &reqwest::Client,
    ) -> ApiResult<Vec<TargetIpfsNodeMessage>>;

    /// Decide which nodes to re-store data on when a store failure occurs.
    ///
    /// Return `errors::IPFS_NODE_CLUSTER_UNHEALTHY` to stop store file.
    /// Could return empty vec.
    async fn decide_store_node_fail_one(&self,
                                        cid: &str,
                                        db_conn: &DatabaseConnection,
                                        reqwest_client: &reqwest::Client,
    ) -> ApiResult<Vec<TargetIpfsNodeMessage>>;

    /// Finish Storage.
    async fn finish_storage(&self, cid: &str) -> ApiResult<()>;
}

/// Message about IPFS node to contact.
#[derive(Clone, Debug)]
#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "Node")]
pub struct TargetIpfsNodeMessage {
    /// Node's id in database.
    pub id: String,
    /// RPC address to contact.
    pub rpc_address: String,
}

