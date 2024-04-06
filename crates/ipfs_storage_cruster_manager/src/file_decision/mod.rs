use std::fmt::Debug;
use std::future::Future;
use sea_orm::DatabaseConnection;
use crate::app::common::ApiResult;

pub mod decision_makers;

/// A trait to make decisions to define file storage strategy.
///
/// A maker should be as stateless as possible.
pub trait FileStorageDecisionMaker: Send + Sync + Debug {
    /// Decide which nodes to store data on.
    ///
    /// Return target `rpc_address` list.
    fn decide_store_node(&mut self,
                         db_conn: &DatabaseConnection,
                         reqwest_client: &reqwest::Client,
    ) -> impl Future<Output=ApiResult<Vec<String>>> + Send
    where Self: Sized;
}

