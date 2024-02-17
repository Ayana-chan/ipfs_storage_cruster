//! # Introduction
//!
//! A struct to record async tasks' execution status with lock-free and async methods.
//!
//! Can host `Future`s and query whether they are **successful**, **failed**, or **running**.
//!
//! - Depend on `tokio` with feature `rt`, so cannot use other async runtimes.
//! - Depend on [scc](https://crates.io/crates/scc) for lock-free and async `HashSet`.
//!
//! Use this crate if:
//! - Easy to generate an **unique** `task_id` (not necessarily be `String`) for a future (task).
//! - Tasks might fail, and then you want to run it again, while you don't want it to success more then once.
//! - Want to record and query all succeeded tasks and failed tasks.
//! - Want to handling every task in the same state (e.g. `success`).
//!
//! And the type of `task_id` should be:
//! - `Eq + Hash + Clone + Send + Sync + 'static`
//! - Cheap to clone (sometimes can use `Arc`).
//!
//! And remember, you can add **anything** in the `Future` to achieve the functionality you want.
//! For example:
//! - Handle your `Result` in `Future`, and then return empty result `Result<(),()>`.
//! - Send a message to a one shot channel at the end of the `Future` to notify upper level that "This task is done".
//! Don't forget to consider using `tokio::spawn` when the channel may not complete sending immediately.
//! - Set other callback functions.
//!
//! > It is recommended to directly look at the source code (about 100 line) if there is any confusion.
//!
//! This crate use three `HashSet` to make it easy to handle all tasks in the same state.
//! But `scc::HashSet` have less contention in `single` access when it grows larger.
//! Therefore, if you don't need handling every task in the same state,
//! just use `scc::HashMap` (`task_id` \-\> `task_status`) to build a simpler implementation,
//! which might have less contention and clone, but more expansive to iterator.
//!
//! # Usage
//!
//! Launch a task with a **unique** `task_id` and a `Future` by [launch](AsyncTasksRecoder::launch).
//!
//! Query the state of the task with its `task_id`
//! by [query_task_state](AsyncTasksRecoder::query_task_state) or [query_task_state_quick](AsyncTasksRecoder::query_task_state_quick).
//!
//! # Theory & Design
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
//! ## Nature
//! ### About Task
//! 1. A task is **launched** by passing a `Future<Output=Result<R, E>>` with unique `task_id`.
//! 2. A task is `real_success` when return `Ok(R)`, and `real_failed` when return `Err(E)`.
//! 3. Different future with **the same `task_id`** is considered **the same task**.
//! 4. The same task **can only `real_success` once**, e.g. a purchase process would never succeed more then once by launching with unique process id as `task_id`.
//!
//! ### About Task State
//! 1. If a task's state is `Success`, it must be `real_success`, i.e. $\text{Success}(id) \rightarrow \text{real\_success}(id)$.
//! 2. If a task's state is `Failed`, it may be in any status, but mostly `real_failed`.
//! 3. If a task's state is `Working`, it may be in any status, but mostly `real_working`.
//!
//! ### About Task State Transition
//! 1. Any task's state can be **queried** at any time, even before the task has been launched.
//! 2. The initial state of the task is `Failed`.
//! 3. Always, when launch a task whose state is `Failed`, it will be `Working` at some future moment.
//! 4. Always, when a task is `Working`, it would eventually be `Fail` or `Success`, i.e. $\Box (\text{Working}(id) \rightarrow \lozenge(\text{Fail}(id) \vee \text{Success}(id)))$.
//! 5. Always, when a task is `Success`, it would be `Success` forever, i.e. $\Box (\text{Success}(id) \rightarrow \Box \text{Success}(id))$.
//!
//! ### Other
//! Relationship between states and containers at [query_task_state](AsyncTasksRecoder::query_task_state).
//!
//! Further propositions and proofs at [AsyncTasksRecoder](AsyncTasksRecoder).
//!
//! Use [query_task_state_quick](AsyncTasksRecoder::query_task_state_quick) for less contention.
//!

use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;

/// Thread-safe.
#[derive(Default, Debug)]
pub struct TaskManager<T>
    where T: Eq + Hash {
    /// hot
    pub working_tasks: scc::HashSet<T>,
    pub success_tasks: scc::HashSet<T>,
    pub failed_tasks: scc::HashSet<T>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TaskStatus {
    /// running or pending
    Working,
    Success,
    Failed,
}

/// Arc was used internally, so after `clone`, the same `TaskManager` was used.
///
/// # Usage
///
/// Launch a task with a **unique** `task_id` and a `Future` by [launch](AsyncTasksRecoder::launch).
///
/// Query the state of the task with its `task_id`
/// by [query_task_state](AsyncTasksRecoder::query_task_state) or [query_task_state_quick](AsyncTasksRecoder::query_task_state_quick).
///
/// # Further Propositions & Proofs
///
/// ## P01
/// **A task (or tasks with the same `task_id`) wouldn't be executed again after first success.**
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
/// - `working_tasks` become a **mutex** (maybe a **spin lock**) for one `task_id`.
/// - `success_tasks` become an atomic boolean, which can only change from false to true.
/// - An execution of a task becomes adding on an atomic int (`count`).
///
/// Therefore, if the `count` is never greater than 1, it means that the task will only be called once.
///
/// ```not_rust
/// let working_task_id = mutex::new();
/// let success_task_id = atomic(false);
/// let count = atomic(0);
/// launch_multi_thread {
///     working_task_id.lock();
///     if success_task_id.get() {
///         exit();
///     }
///     count.add(1);
///     success_task_id.set(true);
///     working_task_id.unlock();
/// }
/// assert_eq!(count.get(), 1);
/// ```
///
/// Obviously, `success_tasks.set(true)` can only be executed once.
/// After that, `exit()` is always called.
/// This results in `count.add(1)` being called only once, too. Q.E.D.
///
/// ## P02
/// **Task failure is not harmful for recorder, and the related operations have been well optimized.**
///
/// Considering the situation of failure, the pseudocode becomes like this:
///
/// ```not_rust
/// let working_task_id = mutex::new();
/// let success_task_id = atomic(false);
/// let failed_task_id = atomic(false); // Initially not in `failed_tasks`, but not important
/// let count = atomic(0);
/// launch_multi_thread {
///     working_task_id.lock();
///     if success_task_id.get() {
///         exit();
///     }
///     // Here should be `real_working`
///     failed_task_id.set(false); // So it shouldn't be `Failed`, just remove from `failed_tasks`
///     count.add(1);
///     if real_success {
///         success_task_id.set(true);
///     } else {
///         // `real_failed`
///         failed_task_id.set(true); // Become `Failed`
///     }
///     working_task_id.unlock();
/// }
/// assert_eq!(count.get(), 1);
/// ```
///
/// In a launch (critical section by `working_tasks`), the initial value of failed is ignored.
/// Therefore, it's not important whether `failed_tasks` changes are atomic for launches.
///
/// From the perspective of `query_task_state`,
/// `failed_tasks` is only meaningful when `task_id` is in it.
///
/// `task_id` is in `failed_tasks` only when it become `real_failed` and before redo (next `real_working`).
/// Very good.
///
/// ## P03
/// **`failed_tasks` is only for optimizing the failure judgment.**
///
/// If there is no `failed_tasks`, and a `task_id` is deleted from all containers if it's failed,
/// what would happen?
///
/// Answer: I have to query `working_tasks` to know it's failed everytime, even no next launch.
/// It would cause more contention.
///
#[derive(Default, Debug, Clone)]
pub struct AsyncTasksRecoder<T>
    where T: Eq + Hash + Clone + Send + 'static {
    task_manager: Arc<TaskManager<T>>,
}

impl<T> AsyncTasksRecoder<T>
    where T: Eq + Hash + Clone + Send + Sync + 'static {
    /// Create a completely new `AsyncTasksRecoder`.
    pub fn new() -> Self {
        AsyncTasksRecoder {
            task_manager: TaskManager {
                working_tasks: scc::HashSet::new(),
                success_tasks: scc::HashSet::new(),
                failed_tasks: scc::HashSet::new(),
            }.into(),
        }
    }

    /// Launch task that returns `Result`.
    ///
    /// The return value of task is ignored, so please use other methods to handle the return value,
    /// such as channel or shared variable.
    ///
    /// - `task_id`: Uniquely mark a task. Different `Future` with **the same `task_id`** is considered **the same task**.
    /// - `task`: A `Future` to be executed automatically.
    pub async fn launch<Fut, R, E>(&self, task_id: T, task: Fut)
        where Fut: Future<Output=Result<R, E>> + Send + 'static,
              R: Send,
              E: Send {
        // insert working -> check success -> remove failed -> work -> insert success/failed -> remove working
        // `working_tasks` play the role of lock
        let res = self.task_manager.working_tasks.insert_async(task_id.clone()).await;
        if res.is_err() {
            // on working
            return;
        }
        // modify status
        if self.task_manager.success_tasks.contains_async(&task_id).await {
            // succeeded
            return;
        }
        // remove from `failed_tasks` if contained
        self.task_manager.failed_tasks.remove_async(&task_id).await;

        // adjust args
        let task_manager = self.task_manager.clone();

        // start
        let _task = tokio::spawn(async move {
            let add_pin_res = task.await;
            if add_pin_res.is_ok() {
                let _ = task_manager.success_tasks.insert_async(task_id.clone()).await;
                task_manager.working_tasks.remove_async(&task_id).await;
            } else {
                let _ = task_manager.failed_tasks.insert_async(task_id.clone()).await;
                task_manager.working_tasks.remove_async(&task_id).await;
            }
        });
    }

    /// Query the state of a task by `task_id`.
    ///
    /// Query priority of containers : `success_tasks` -> `failed_tasks` -> `working_tasks`.
    ///
    /// **NOTE**: `working_tasks` usually has more contention.
    ///
    /// If not found in all tasks, be `Failed`.
    /// Only occurs before the launch or in a very short period of time after the first launch.
    ///
    /// Note, if `T` is `String`, then parameter `task_id` would be `&String` instead of `&str`.
    pub async fn query_task_state(&self, task_id: &T) -> TaskStatus {
        if self.task_manager.success_tasks.contains_async(task_id).await {
            return TaskStatus::Success;
        }

        if self.task_manager.failed_tasks.contains_async(task_id).await {
            return TaskStatus::Failed;
        }

        if self.task_manager.working_tasks.contains_async(task_id).await {
            return TaskStatus::Working;
        }

        TaskStatus::Failed
    }

    /// Return `Working` if not in either `success_tasks` or `failed_tasks`.
    ///
    /// No query in `working_tasks`, so less contention.
    ///
    /// Even when the `task_id`'s launch have not occurred, return `Working`.
    ///
    /// Use this if you are certain that the task's launch must occur at some point in the past or future,
    /// and don't care about when the launch occurs
    /// (because first launch always turns into `Working` at some point).
    pub async fn query_task_state_quick(&self, task_id: &T) -> TaskStatus {
        if self.task_manager.success_tasks.contains_async(task_id).await {
            return TaskStatus::Success;
        }

        if self.task_manager.failed_tasks.contains_async(task_id).await {
            return TaskStatus::Failed;
        }

        TaskStatus::Working
    }

    /// Get a cloned `Arc` of `task_manager`.
    /// Then you can do anything you want (Every containers are public).
    pub fn get_raw_task_manager(&self) -> Arc<TaskManager<T>> {
        self.task_manager.clone()
    }

    /// Get a reference of `success_tasks`.
    pub fn get_success_tasks_ref(&self) -> &scc::HashSet<T> {
        &self.task_manager.success_tasks
    }

    /// Get a reference of `working_tasks`.
    pub fn get_working_tasks_ref(&self) -> &scc::HashSet<T> {
        &self.task_manager.working_tasks
    }

    /// Get a reference of `failed_tasks`.
    pub fn get_failed_tasks_ref(&self) -> &scc::HashSet<T> {
        &self.task_manager.failed_tasks
    }

    pub fn retain_success_tasks(&self) {

    }

    pub fn delete_failed_task(&self) -> bool{
        false
    }

    pub fn clear_failed_tasks(&self) {

    }
}



