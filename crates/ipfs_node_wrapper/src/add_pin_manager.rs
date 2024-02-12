use std::sync::Arc;
use crate::ipfs_client::IpfsClient;

#[derive(Default, Debug)]
pub struct TaskManager {
    working_tasks: scc::HashSet<String>,
    success_tasks: scc::HashSet<String>,
    failed_tasks: scc::HashSet<String>,
}

#[derive(Default, Debug)]
pub struct AddPinManager {
    task_manager: Arc<TaskManager>,
}

impl AddPinManager {
    pub fn new() -> Self {
        AddPinManager {
            task_manager: TaskManager::default().into()
        }
    }

    /// Start add pin in background. Return immediately.
    pub async fn launch(&self, ipfs_client: &IpfsClient, cid: &str, name: Option<&str>) {
        // modify pin status
        if self.task_manager.success_tasks.contains_async(cid).await {
            // succeeded
            return;
        }
        let res = self.task_manager.working_tasks.insert_async(cid.to_string()).await;
        if res.is_err() {
            // on working
            return;
        }
        // remove from `failed_tasks` if contained
        self.task_manager.failed_tasks.remove_async(cid).await;
        // Although a pin may be in `failed_tasks` while it has been pinned successfully, it's ok to pin again. A `pin verify` is not necessary.

        // adjust args
        let task_manager = self.task_manager.clone();
        let ipfs_client = ipfs_client.clone();
        let cid = cid.to_string();
        let name = name.map(String::from);

        // start
        let _task = tokio::spawn(async move {
            let add_pin_res = ipfs_client
                .add_pin_recursive(
                    &cid,
                    name.as_deref(),
                ).await;
            // Guarantee any launched cid can be found in one of the sets.
            // But it causes a copy of cid.
            // TODO 优化这里的string clone
            if let Ok(_success_res) = add_pin_res {
                let _ = task_manager.success_tasks.insert_async(cid.clone()).await;
                task_manager.working_tasks.remove_async(&cid).await;
            } else {
                let _ = task_manager.failed_tasks.insert_async(cid.clone()).await;
                task_manager.working_tasks.remove_async(&cid).await;
            }
        });
    }

    /// Get a cloned Arc of task_manager.
    pub fn task_manager(&self) -> Arc<TaskManager> {
        self.task_manager.clone()
    }
}

