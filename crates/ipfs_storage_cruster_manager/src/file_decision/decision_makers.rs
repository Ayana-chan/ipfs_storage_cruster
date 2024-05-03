use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use axum::async_trait;
use reqwest::Client;
use sea_orm::DatabaseConnection;
#[allow(unused_imports)]
use tracing::{error, info, warn};
use crate::imports::dao_imports::*;
use crate::app::common::ApiResult;
use crate::app::{services, errors, daos};
use crate::file_decision::{FileDownloadDecisionMaker, FileStorageDecisionMaker, TargetAdminIpfsNodeMessage, TargetPublicWrapperMessage};

/// Simple decision maker of `FileStoreDecision`.
pub struct RandomFileStorageDecisionMaker {
    /// Store the status of tasks. HashSet<String> is the set of stored nodes.
    task_map: scc::HashMap<String, HashSet<String>>,
}

impl RandomFileStorageDecisionMaker {
    pub fn new() -> Self {
        RandomFileStorageDecisionMaker {
            task_map: scc::HashMap::new(),
        }
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
                               cid: &str,
                               db_conn: &DatabaseConnection,
                               _reqwest_client: &Client)
                               -> ApiResult<Vec<TargetAdminIpfsNodeMessage>> {
        const STORE_NODE_NUM: usize = 2;
        let available_nodes = Node::find()
            .filter(node::Column::NodeStatus.ne(sea_orm_active_enums::NodeStatus::Offline))
            .into_partial_model::<TargetAdminIpfsNodeMessage>()
            .all(db_conn).await
            .map_err(services::db::handle_db_error)?;
        let available_node_num = available_nodes.len();

        // It's ok when `available_node_num` is less than node_num.
        let decide_result = fastrand::choose_multiple(available_nodes.into_iter(), STORE_NODE_NUM);

        let decision = decide_result.iter().map(|v| v.id.clone()).collect();
        let res = self.task_map.insert_async(cid.to_owned(), decision).await;
        // Return existed decision if the cid is already in `task_map`. It should not happen.
        if let Err(e) = res {
            error!("decide_store_node called when the cid {} is still on storing.", e.0);
            return Err(errors::SYSTEM_EXECUTION_ERROR.clone_to_error());
        }

        info!("Find {available_node_num} available IPFS nodes. Choose {STORE_NODE_NUM} node randomly. Result: {decide_result:?}");
        Ok(decide_result)
    }

    // TODO 哪个节点失败了？
    #[tracing::instrument(skip_all)]
    async fn decide_store_node_fail_one(&self,
                                        cid: &str,
                                        db_conn: &DatabaseConnection,
                                        _reqwest_client: &Client)
                                        -> ApiResult<Vec<TargetAdminIpfsNodeMessage>> {
        // retry would be executed one by one
        let pre_decision_entry = self.task_map.get_async(cid).await;
        match pre_decision_entry {
            None => {
                error!("decide_store_node_fail_one called when the cid {cid} is not in task_map");
                Err(errors::SYSTEM_EXECUTION_ERROR.clone_to_error())
            }
            Some(mut pre_decision_entry) => {
                let pre_decision = pre_decision_entry.get();
                let available_nodes = Node::find()
                    .filter(node::Column::NodeStatus.ne(sea_orm_active_enums::NodeStatus::Offline))
                    .filter(node::Column::Id.is_not_in(pre_decision))
                    .into_partial_model::<TargetAdminIpfsNodeMessage>()
                    .all(db_conn).await
                    .map_err(services::db::handle_db_error)?;
                let available_node_num = available_nodes.len();

                let decide_result = fastrand::choice(available_nodes);

                if let Some(decide_result) = decide_result {
                    // record the decision
                    let decision = decide_result.id.clone();
                    let decision_set = pre_decision_entry.get_mut();
                    let res = decision_set.insert(decision);
                    assert!(res, "New decision caused a conflict. Decision: {decide_result:?}");

                    info!("Find {available_node_num} available IPFS nodes. Choose 1 node randomly. Result: {decide_result:?}");
                    Ok(vec![decide_result])
                } else {
                    warn!("decide_store_node_fail_one return an empty result");
                    Ok(vec![])
                }
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn finish_storage(&self, cid: &str) -> ApiResult<()> {
        let res = self.task_map.remove_async(cid).await;
        if res.is_none() {
            warn!("finish_storage called  when the cid {cid} is not in task_map");
            return Err(errors::SYSTEM_EXECUTION_ERROR.clone_to_error());
        }
        Ok(())
    }
}

/// Simple decision maker of `FileStoreDecision`.
pub struct RandomFileDownloadDecisionMaker {}

impl RandomFileDownloadDecisionMaker {
    pub fn new() -> Self {
        RandomFileDownloadDecisionMaker {}
    }
}

impl Debug for RandomFileDownloadDecisionMaker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random File Download Decision Maker")
    }
}

#[async_trait]
impl FileDownloadDecisionMaker for RandomFileDownloadDecisionMaker {
    #[tracing::instrument(skip_all)]
    async fn decide_download_node(&self,
                                  cid: &str,
                                  db_conn: &DatabaseConnection,
                                  _reqwest_client: &Client
    ) -> ApiResult<TargetPublicWrapperMessage> {
        let available_nodes = daos::find_nodes_with_pin_cid(cid, db_conn)
            .await.map_err(services::db::handle_db_error)?;
        let available_node_num = available_nodes.len();

        let decide_result = fastrand::choice(available_nodes);
        if let Some(decide_result) = decide_result {
            info!("Find {available_node_num} available IPFS nodes. Result: {decide_result:?}");
            Ok(decide_result)
        } else {
            Err(errors::IPFS_NODE_CLUSTER_UNHEALTHY.clone_to_error())
        }
    }
}

