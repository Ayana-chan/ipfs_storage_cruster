use std::fmt::{Debug, Formatter};
use axum::async_trait;
use reqwest::Client;
use sea_orm::DatabaseConnection;
use tracing::info;
use crate::imports::dao_imports::*;
use crate::app::common::ApiResult;
use crate::app::services;
use crate::file_decision::FileStorageDecisionMaker;

/// Default decision maker of `FileStoreDecision`.
pub struct RandomFileStorageDecisionMaker {}

impl RandomFileStorageDecisionMaker {
    pub fn new() -> Self {
        RandomFileStorageDecisionMaker {}
    }
}

impl Debug for RandomFileStorageDecisionMaker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random File Storage Decision Maker")
    }
}

#[async_trait]
impl FileStorageDecisionMaker for RandomFileStorageDecisionMaker {
    #[tracing::instrument(skip_all)]
    async fn decide_store_node(&self,
                               db_conn: &DatabaseConnection,
                               _reqwest_client: &Client)
                               -> ApiResult<Vec<String>> {
        let available_nodes = Node::find()
            .select_only()
            .columns([node::Column::Id, node::Column::RpcAddress])
            .filter(node::Column::NodeStatus.ne(sea_orm_active_enums::NodeStatus::Offline))
            .all(db_conn).await
            .map_err(services::db::handle_db_error)?;

        let available_node_num = available_nodes.len();
        const STORE_NODE_NUM: usize = 3;
        // It's ok when `available_node_num` is less than 3.
        let decide_result = fastrand::choose_multiple(available_nodes.into_iter(), STORE_NODE_NUM);
        let rpc_addrs = decide_result.into_iter().map(|v| v.rpc_address).collect();
        info!("Find {available_node_num} available IPFS nodes. Choose {STORE_NODE_NUM} node randomly. Result: {rpc_addrs:?}");
        Ok(rpc_addrs)
    }
}

