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

//TODO 任何时候都可以重新launch，只不过大部分就可以直接拒绝
//TODO 如果失败的话，task的拷贝如何复用？甚至有没有可能复用future？
/// # Proof
///
/// ## P01
/// **A task (or tasks with the same `task_id`) wouldn't be executed again after one success.** \
///
/// When a task fail, it wouldn't break anything. Failure just means the task could be launched again,
/// so if this proposition (**P01**) is true, there is almost nothing to worry about.
/// For further discussion, please refer to **P02**. \
///
/// `working_tasks` play the role of lock,
/// which allow tasks with the same `task_id` to execute remaining codes (after `insert` & before `remove`) only once.
/// And before `remove` from `working_tasks`, the succeeded `task_id` has been in `success_tasks`.
///
/// In the case of success alone, an equivalent pseudocode can be obtained \
/// - `working_tasks` become a **mutex** for one `task_id`. \
/// - `success_tasks` become an atomic boolean, which can only change from false to true. \
/// - An execution of a task becomes adding on an atomic int (`count`).
/// Therefore, if the `count` is never greater than 1, it means that the task will only be called once.
///
/// ```not_rust
/// let working_tasks = mutex::new();
/// let success_tasks = atomic(false);
/// let count = atomic(0);
/// loop_multi_thread {
///     working_tasks.lock();
///     if success_tasks.get() {
///         exit();
///     }
///     count.add(1);
///     success_tasks.set(true);
///     working_tasks.unlock();
/// }
/// assert_eq!(count.get(), 1);
/// ```
/// Obviously, `success_tasks.set(true)` can only be executed once.
/// After that, `exit()` is always called.
/// This results in `count.add(1)` being called only once, too. Q.E.D.
///
/// ## P02
/// **Task failure is not harmful, and the related operations have been well optimized** \
/// `failed task` is only for optimizing the failure judgment. TODO
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

