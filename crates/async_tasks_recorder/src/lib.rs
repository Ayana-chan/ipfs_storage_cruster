//! It is recommended to directly look at the source code if there is any confusion.
//!
//! ## Abstract Model
//! Here is the three-level structure for thinking about tasks' status:
//! - Level 0: `real_failed`, `real_working`, `real_success` : **Exact status** of the tasks in the CPU (seen by God).
//! - Level 1: `failed_tasks`, `working_tasks`, `success_tasks` : **Containers** to store `task_id`s (a `task_id` can be stored in 0 to 2 containers simultaneously).
//! - Level 2: `Failed`, `Working`, `Success` : **States** of the task that could be obtained by `query_task_state`.
//!
//! ## State Transition Diagram
//! `Failed` \<\-\-\-\> `Working` \-\-\-\-\> `Success`
//!
//! ## Usage & Nature
//! ### About Task
//! 1. A task is **launched** by passing a `Future<Output=Result<R, E>>` with unique `task_id`.
//! 2. A task is `real_success` when return `Ok(R)`, and `real_failed` when return `Err(E)`.
//! 3. Different future with **the same `task_id`** is considered **the same task**.
//! 4. The same task **can only `real_success` once**, e.g. a purchase process would never succeed more then once by launching with unique process id as `task_id`.
//!
//! ### About Task State
//! 1. If a task's state is `Success`, it must be `real_success`, i.e. $\text{Success}(id) \rightarrow \text{real\_success}(id)$.
//! 2. If a task's state is `Failed`, it may be in any status, but mostly `real_failed`.
//! 3. If a task's state is `Working`, it may be in any status, but mostly `working`.
//!
//! ### About Task State Transition
//! 1. Any task's state can be **queried** at any time, even before the task has been launched.
//! 2. The initial state of the task is `Failed`.
//! 3. Always, when launch a task whose state is `Failed`, it will be `Working` at some future moment.
//! 4. Always, when a task is `Working`, it would eventually be `Fail` or `Success`, i.e. $\Box (\text{Working}(id) \rightarrow \lozenge(\text{Fail}(id) \vee \text{Success}(id)))$.
//! 5. Always, when a task is `Success`, it would be `Success` forever, i.e. $\Box (\text{Success}(id) \rightarrow \Box \text{Success}(id))$.
//!
//! ### Other
//! Relationship between states and containers at [query_task_state](crate::AsyncTasksRecoder::query_task_state).
//!
//! Further propositions and proofs at [AsyncTasksRecoder](crate::AsyncTasksRecoder).
//!

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
    Success,
    Failed,
}

//TODO 添加机制，在success更新的时候能直接拿到，最大开销是每个task一个通道。
//TODO 如果失败的话，task的拷贝如何复用？甚至有没有可能复用future？
/// Safe to clone.
/// # Further Propositions & Proofs
///
/// ## P01
/// **A task (or tasks with the same `task_id`) wouldn't be executed again after one success.**
///
/// When a task fail, it wouldn't break anything. Failure just means the task could be launched again,
/// so if this proposition (**P01**) is true, there is almost nothing to worry about.
/// For further discussion, please refer to **P02**.
///
/// From now on, only consider the situation of success.
///
/// `working_tasks` play the role of lock,
/// which allow tasks with the same `task_id` to execute remaining codes (after `insert` & before `remove`) only once.
/// And before `remove` from `working_tasks`, the succeeded `task_id` has been in `success_tasks`.
///
/// An equivalent pseudocode can be obtained.
/// - `working_tasks` become a **mutex** for one `task_id`.
/// - `success_tasks` become an atomic boolean, which can only change from false to true.
/// - An execution of a task becomes adding on an atomic int (`count`).
///
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
/// **Task failure is not harmful, and the related operations have been well optimized.**
///
/// `failed task` is only for optimizing the failure judgment. TODO proof
#[derive(Default, Debug, Clone)]
pub struct AsyncTasksRecoder {
    task_manager: Arc<TaskManager>,
}

impl AsyncTasksRecoder {
    pub fn new() -> Self {
        AsyncTasksRecoder {
            task_manager: TaskManager::default().into()
        }
    }

    /// Launch task. \ TODO redo
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

    // TODO 判断命题
    /// Priority: success_tasks -> failed_tasks -> working_tasks. \
    /// If not found in all tasks, be `Failed`.
    pub async fn query_task_state(&self, cid: &str) -> TaskStatus {
        if self.task_manager.success_tasks.contains_async(cid).await {
            return TaskStatus::Success;
        }

        if self.task_manager.failed_tasks.contains_async(cid).await {
            return TaskStatus::Failed;
        }

        if self.task_manager.working_tasks.contains_async(cid).await {
            return TaskStatus::Working;
        }

        TaskStatus::Failed
    }

    /// Get a cloned `Arc` of `task_manager`. Then you can do anything you want.
    pub fn get_task_manager(&self) -> Arc<TaskManager> {
        self.task_manager.clone()
    }
}

