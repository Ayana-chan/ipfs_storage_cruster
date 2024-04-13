use std::fmt::{Debug, Formatter};
use axum::async_trait;
use reqwest::Client;
use sea_orm::DatabaseConnection;
use tracing::info;
use crate::imports::dao_imports::*;
use crate::app::common::ApiResult;
use crate::app::{services, errors};
use crate::file_decision::{FileStorageDecisionMaker, TargetIPFSNodeMessage};

/// Default decision maker of `FileStoreDecision`.
pub struct RandomFileStorageDecisionMaker {}

impl RandomFileStorageDecisionMaker {
    pub fn new() -> Self {
        RandomFileStorageDecisionMaker {}
    }

    /// Randomly choose nodes which are not `Offline`. Might return empty list.
    async fn random_choose_node(node_num: usize,
                                db_conn: &DatabaseConnection,
    ) -> ApiResult<Vec<TargetIPFSNodeMessage>> {
        let available_nodes = Node::find()
            .filter(node::Column::NodeStatus.ne(sea_orm_active_enums::NodeStatus::Offline))
            .into_partial_model::<TargetIPFSNodeMessage>()
            .all(db_conn).await
            .map_err(services::db::handle_db_error)?;

        let available_node_num = available_nodes.len();
        // It's ok when `available_node_num` is less than 3.
        let decide_result = fastrand::choose_multiple(available_nodes.into_iter(), node_num);
        info!("Find {available_node_num} available IPFS nodes. Choose {node_num} node randomly. Result: {decide_result:?}");
        Ok(decide_result)
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
                               -> ApiResult<Vec<TargetIPFSNodeMessage>> {
        const STORE_NODE_NUM: usize = 3;
        let target = Self::random_choose_node(STORE_NODE_NUM, db_conn).await?;
        if target.is_empty() {
            return Err(errors::IPFS_NODE_CLUSTER_UNHEALTHY.clone_to_error());
        }
        Ok(target)
    }

    #[tracing::instrument(skip_all)]
    async fn decide_store_node_fail_one(&self,
                                        db_conn: &DatabaseConnection,
                                        _reqwest_client: &Client)
                                        -> ApiResult<Vec<TargetIPFSNodeMessage>> {
        let target = Self::random_choose_node(1, db_conn).await?;
        if target.is_empty() {
            return Err(errors::IPFS_NODE_CLUSTER_UNHEALTHY.clone_to_error());
        }
        Ok(target)
    }

    #[tracing::instrument(skip_all)]
    async fn on_add_new_node(&self) {
        info!("RandomFileStorageDecisionMaker: on_add_new_node");
    }
}

