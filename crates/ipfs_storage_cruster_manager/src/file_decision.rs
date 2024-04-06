use sea_orm::DatabaseConnection;

pub struct FileStoreDecision<M> {
    /// Decision maker.
    pub(crate) maker: M,
}

impl<M> FileStoreDecision<M>
    where M: FileStoreDecisionMaker {
    /// Create with a customized decision maker.
    pub fn with_decision_maker(maker: M) -> Self {
        FileStoreDecision {
            maker
        }
    }
}

impl<M> Default for FileStoreDecision<M> {
    fn default() -> Self {
        todo!()
    }
}

pub trait FileStoreDecisionMaker {
    /// Decide which nodes to store data on.
    ///
    /// Return target `rpc_address` list.
    fn decide_store_node(&mut self,
                         db_conn: &DatabaseConnection,
                         reqwest_client: &reqwest::Client
    ) -> impl std::future::Future<Output = Vec<String>> + Send;
}
