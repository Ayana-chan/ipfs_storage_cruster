use std::sync::Arc;
use crate::ipfs_client::ReqwestIpfsClient;

#[derive(Default, Debug)]
pub struct TaskManager {
    /// hot
    working_tasks: scc::HashSet<String>,
    success_tasks: scc::HashSet<String>,
    failed_tasks: scc::HashSet<String>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TaskStatus {
    /// pinning or queued
    Pinning,
    /// success
    Pinned,
    Failed,
}

/// Safe to clone.
#[derive(Default, Debug, Clone)]
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
    pub async fn launch(&self, ipfs_client: &ReqwestIpfsClient, cid: &str, name: Option<&str>) {
        let ipfs_client = ipfs_client.clone();
        let cid_backup = cid;
        let cid = cid.to_string();
        let name = name.map(String::from);
        let add_pin_task = Box::pin(async move {
            let add_pin_res = ipfs_client
                .add_pin_recursive(
                    &cid,
                    name.as_deref(),
                ).await;
            // TODO log error
            return if let Ok(_success_res) = add_pin_res {
                Ok(())
            } else {
                Err(())
            };
        });
        self.launch_core(cid_backup, add_pin_task).await;
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

        return TaskStatus::Failed;
    }

    /// Get a cloned `Arc` of `task_manager`.
    pub fn task_manager(&self) -> Arc<TaskManager> {
        self.task_manager.clone()
    }

    async fn launch_core(&self, cid: &str, add_pin_task: std::pin::Pin<Box<dyn std::future::Future<Output=Result<(), ()>> + Send>>) {
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
        let cid = cid.to_string();

        // start
        let _task = tokio::spawn(async move {
            let add_pin_res = add_pin_task.await;
            // Guarantee any launched cid can be found in one of the sets.
            // But it causes a copy of cid.
            // TODO 优化这里的string clone
            if add_pin_res.is_ok() {
                let _ = task_manager.success_tasks.insert_async(cid.clone()).await;
                task_manager.working_tasks.remove_async(&cid).await;
            } else {
                // more contention or more memory copy?
                task_manager.working_tasks.remove_async(&cid).await;
                let _ = task_manager.failed_tasks.insert_async(cid).await;
            }
        });
    }
}


#[cfg(test)]
mod tests {
    use std::future::Future;
    use super::*;

    static DEFAULT_CHECK_INTERVAL_MS: u64 = 5;
    static DEFAULT_CHECK_TIMEOUT_MS: u128 = 1000;

    enum RuntimeType {
        CurrentThread,
        CustomThread(usize),
        MultiThread,
    }

    #[test]
    fn test_add_pin_manager_basic() {
        do_async_test(
            RuntimeType::MultiThread,
            test_add_pin_manager_basic_core,
        );
    }

    // #[test]
    // fn test_add_pin_manager_serial() {}
    //
    // #[test]
    // fn test_add_pin_manager_random() {}

    // core functions ---------------------------------------------------------------

    async fn test_add_pin_manager_basic_core() {
        let manager = AddPinManager::new();
        manager.launch_core("t1", generate_empty_task()).await;
        check_success(&manager, "t1", None, None).await;
    }

    // tools ------------------------------------------------------------------------

    fn do_async_test<F, Fut>(runtime_type: RuntimeType, test_func: F)
        where F: FnOnce() -> Fut,
              Fut: Future<Output=()>, {
        let runtime;
        match runtime_type {
            RuntimeType::CurrentThread => {
                runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build().unwrap();
            }
            RuntimeType::CustomThread(thread_num) => {
                runtime = tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(thread_num)
                    .enable_all()
                    .build().unwrap();
            }
            RuntimeType::MultiThread => {
                runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build().unwrap();
            }
        }

        runtime.block_on(async {
            test_func().await;
        })
    }

    async fn check_success_once(manager: &AddPinManager, cid: &str) -> bool {
        manager.get_task_status(cid).await == TaskStatus::Pinned
    }

    /// Err when timeout
    async fn check_success(manager: &AddPinManager, cid: &str, interval_ms: Option<u64>, timeout_ms: Option<u128>) {
        let interval_ms = interval_ms.unwrap_or(DEFAULT_CHECK_INTERVAL_MS.clone());
        let timeout_ms = timeout_ms.unwrap_or(DEFAULT_CHECK_TIMEOUT_MS.clone());

        let start_time = std::time::Instant::now();

        loop {
            // timeout
            if start_time.elapsed().as_millis() >= timeout_ms {
                panic!("Timeout before success. cid: {:?}", cid);
            }

            if check_success_once(manager, cid).await {
                return;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms.clone())).await;
        }
    }

    fn generate_empty_task() -> std::pin::Pin<Box<dyn std::future::Future<Output=Result<(), ()>> + Send>> {
        Box::pin(async {
            Ok(())
        })
    }
}

