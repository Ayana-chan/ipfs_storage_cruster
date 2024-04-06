pub struct FileStoreDecision<M> {
    pub(crate) maker: M,
}

impl<M> FileStoreDecision<M>
    where M: FileStoreDecisionMaker {
    pub fn with_decision_maker(maker: M) -> Self {
        FileStoreDecision {
            maker
        }
    }
}

pub trait FileStoreDecisionMaker {
    /// Decide which nodes to store data on.
    ///
    /// Return target `rpc_address` list.
    fn decide_store_node(&mut self) -> impl std::future::Future<Output = Vec<String>> + Send;
}
