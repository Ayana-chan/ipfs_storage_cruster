//! `failed` <---> `working` ----> `success`

use std::future::Future;
use std::sync::Arc;

/// Thread-safe.
#[derive(Default, Debug)]
pub struct TaskManager {
    /// hot
    pub working_tasks: scc::HashSet<String>,
    pub success_tasks: scc::HashSet<String>,
    pub failed_tasks: scc::HashSet<String>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TaskStatus {
    /// running or pending
    Working,
    Pinned,
    Failed,
}

/// Safe to clone.
#[derive(Default, Debug, Clone)]
pub struct AsyncTasksRecoder {
    task_manager: Arc<TaskManager>,
}

/// # Proof
///
/// ## P01
/// **A task (or tasks with the same `task_id`) wouldn't be executed again after one success.** \
/// `working_tasks` play the role of lock,
/// which only allow tasks with the same `task_id` to execute remaining codes before `remove` from `working_tasks`.
/// And before `remove` from `working_tasks`, the succeeded `task_id` has been in `success_tasks`.
///
/// When a task fail, it wouldn't break anything. Failure just means the task could be launched again,
/// so if the proposition is true, there is almost nothing to worry about.
///
/// ```not_rust
/// let success_tasks = atomic(0);
/// let count = atomic(0);
/// loop_multi_thread {
///     working_tasks.lock();
///     if success_tasks.get() == 1 exit();
///     count.add(1);
///     success_tasks.set(1);
///     working_tasks.unlock();
/// }
/// assert_eq!(count.get(), 1);
/// ```
///
/// ```not_rust
/// let success_tasks = atomic(0);
/// let count = atomic(0);
/// loop_multi_thread {
///     working_tasks.lock();
///     if success_tasks.get() == 1 exit();
///     success_tasks.set(1);
///     count.add(1);
///     working_tasks.unlock();
/// }
/// assert_eq!(count.get(), 1);
/// ```
///
/// ## P02
/// ** **`failed task` is only for optimizing the failure judgment.
impl AsyncTasksRecoder {
    pub fn new() -> Self {
        AsyncTasksRecoder {
            task_manager: TaskManager::default().into()
        }
    }

    /// Launch task. \
    /// `task`: an async closure to add pin (truly interact with IPFS without modifying manager).
    pub async fn launch<Fut, R, E>(&self, task_id: &str, task: Fut)
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        // insert working -> check success -> remove failed -> work -> insert success -> remove working
        // `working_tasks` play the role of lock
        let res = self.task_manager.working_tasks.insert_async(task_id.to_string()).await;
        if res.is_err() {
            // on working
            return;
        }
        // modify status
        if self.task_manager.success_tasks.contains_async(task_id).await {
            // succeeded
            return;
        }
        // remove from `failed_tasks` if contained
        self.task_manager.failed_tasks.remove_async(task_id).await;

        // adjust args
        let task_manager = self.task_manager.clone();
        let cid = task_id.to_string();

        // start
        let _task = tokio::spawn(async move {
            let add_pin_res = task.await;
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

    /// success_tasks -> failed_tasks -> working_tasks
    /// If not found in all tasks, be `Failed`.
    pub async fn get_task_status(&self, cid: &str) -> TaskStatus {
        if self.task_manager.success_tasks.contains_async(cid).await {
            return TaskStatus::Pinned;
        }

        if self.task_manager.failed_tasks.contains_async(cid).await {
            return TaskStatus::Failed;
        }

        if self.task_manager.working_tasks.contains_async(cid).await {
            return TaskStatus::Working;
        }

        return TaskStatus::Failed;
    }

    /// Get a cloned `Arc` of `task_manager`. Then you can do anything you want.
    pub fn get_task_manager(&self) -> Arc<TaskManager> {
        self.task_manager.clone()
    }
}

