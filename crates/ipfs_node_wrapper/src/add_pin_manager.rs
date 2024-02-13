use std::sync::Arc;
use crate::ipfs_client::IpfsClient;

#[derive(Default, Debug)]
pub struct TaskManager {
    /// hot
    working_tasks: scc::HashSet<String>,
    success_tasks: scc::HashSet<String>,
    failed_tasks: scc::HashSet<String>,
}

pub enum  TaskStatus {
    /// pinning or queued
    Pinning,
    /// success
    Pinned,
    Failed,
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

    /// Start add pin in background. Return immediately. \
    /// ## Ugly Proof
    /// **Environment Propositions**: \
    /// 1. The status will be queried multiple times, and may only retry in case of `Failed`. \
    /// 2. For network reasons, a task may become `Failed` while it has been pinned successfully in IPFS node. \
    /// 3. Adding a pin that has done successfully in IPFS node is ok, with almost no consumption. \
    /// 4. Must not to report `Pinned` when is not `success`. \
    /// 5. A task must eventually be `Pinned` or `Failed`. (ignored) \
    /// 6. A `Pinned` task must not turn into any other status in future.
    ///
    /// **Implementation Propositions**: \
    /// 1. A task is in `success_tasks` only when it's `success`. \
    /// 2. If a task is in `failed_tasks`, it may be in any status, but mostly `failed`. \
    /// 3. If a task is in `working_tasks`, it may be in any status, but mostly `working`. \
    /// 4. A task was reported as `Pinned` when and only when it's in `success_tasks`. \
    /// 5. A task was reported as `Failed` when and only when it's in `failed_tasks` but not in `success_tasks`, or not in any `tasks`. \
    /// 6. A task was reported as `Pinning` when and only when it's in `working_tasks` while not in either `success_tasks` or `failed_tasks`. \
    ///
    /// **Derived Propositions**: \
    /// 1. A task was reported as `Pinned` only when it's `success`. (I1, I4) \
    /// 2. Logic about success is correct. (E4, D1) \
    /// 3. It's acceptable to be a slightly more `Failed`, but its priority should be lower then `Pinned`. (E1, E3, E6) \
    /// 4. It's impossible to avoid reporting a task as `Failed` when it's `working` or `success`. (E2, I2, I3, I5) \
    /// 5. A task must not be `Failed` when it's in `success_tasks`. (E6, I4) \
    /// 6. A task should always be in one of the `tasks` unless it's `failed`. (I5)
    pub async fn launch(&self, ipfs_client: &IpfsClient, cid: &str, name: Option<&str>) {
        // check success -> insert working -> remove failed -> work -> insert success -> remove working
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
        // Although , it's ok to pin again. A `pin verify` is not necessary.

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
                // more contention or more memory copy?
                task_manager.working_tasks.remove_async(&cid).await;
                let _ = task_manager.failed_tasks.insert_async(cid).await;
            }
        });
    }

    /// success -> failed -> working
    /// If not found in all tasks, be `Failed`.
    pub async fn get_task_status(&self, cid: &str) -> TaskStatus {
        if self.task_manager.success_tasks.contains_async(cid).await {
            return TaskStatus::Pinned;
        }

        if self.task_manager.failed_tasks.contains_async(cid).await {
            return TaskStatus::Failed;
        }

        if self.task_manager.working_tasks.contains_async(cid).await {
            return TaskStatus::Pinning;
        }

        return TaskStatus::Failed
    }

    /// Get a cloned `Arc` of `task_manager`.
    pub fn task_manager(&self) -> Arc<TaskManager> {
        self.task_manager.clone()
    }
}

