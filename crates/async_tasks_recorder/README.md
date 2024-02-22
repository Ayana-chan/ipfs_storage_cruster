# Introduction

A struct for recording execution status of async tasks with lock-free and async methods.

Functions:
- Able to host `Future`s and query whether they are **not found**, **successful**, **failed**, or **running**.
- Able to host `Future`s to revoke the succeeded `Future`s and make them **not found**.

Dependency:
- Depend on `tokio` with feature `rt`, so cannot use other async runtimes.
- Depend on [scc](https://crates.io/crates/scc) for lock-free and async `HashSet`.

Use this crate if:
- Easy to generate an **unique** `task_id` (not necessarily `String`) for a future (task).
- Don't want tasks with the same `task_id` to succeed more then once.
- Want to record and query all succeeded tasks and failed tasks.
- Want to handling every task in the same state (not just focus on one state).

[Example](https://github.com/Ayana-chan/ipfs_storage_cruster/tree/master/crates/async_tasks_recorder/examples).

A recorder can only use one `task_id` type. The type of `task_id` should be:
- `Eq + Hash + Clone + Send + Sync + 'static`
- Cheap to clone (sometimes can use `Arc`).

## When Shouldn't Use This Crate

This crate use **three `HashSet`** to make it easy to operate all tasks in the same state.
However, `scc::HashSet` have less contention in **single** access when it grows larger.

Therefore, if you don't need operating every task in the same state,
then just use `scc::HashMap` (`task_id` \-\> `task_status`) to build a simpler implementation,
which might have less contention and cloning, but more expansive to iterate.
And the `scc::HashMap::update_async` could be a powerful tool for atomic operations.

You should also avoid using this crate if you just want to handle every tasks in only one state.
For example, if you just want to manage the failed tasks,
then you should use `scc::HashMap` to record tasks' states,
and insert the failed tasks into a external `Arc<scc::HashSet>` in `Future`.

# Usage

Launch a task with a **unique** `task_id` and a `Future` by [launch](AsyncTasksRecoder::launch).

Query the state of the task with its `task_id`
by [query_task_state](AsyncTasksRecoder::query_task_state) or [query_task_state_quick](AsyncTasksRecoder::query_task_state_quick).


## Skills

Remember that you can add **anything** in the `Future` to achieve the functionality you want.
For example:
- Handle your `Result` in `Future`, and then return empty result `Result<(),()>`.
- Send a message to a one shot channel at the end of the `Future` to notify upper level that "This task is done".
  Don't forget to consider using `tokio::spawn` when the channel may not complete sending immediately.
- Set other callback functions.

It's still efficient to store metadata of tasks at external `scc::HashMap` (`task_id` \-\> metadata).

> It is recommended to directly look at the source code (about 150 line) if there is any confusion.

# Theory & Design

## Abstract Model
Here is the three-level structure for thinking about tasks' status:
- Level 0: `real_none`, `real_failed`, `real_working`, `real_success` : **Exact status** of the tasks in the CPU (seen by God).
- Level 1: `failed_tasks`, `working_tasks`, `success_tasks` : **Containers** to store `task_id`s (a `task_id` can be stored in 0 to 2 containers simultaneously).
- Level 2: `Not Found`, `Failed`, `Working`, `Success` : **States** of the task that could be obtained by `query_task_state`.

## State Transition Diagram
- `Not Found` \-\-\-\-\> `Working` (first launch)
- `Working` \-\-\-\-\> `Failed` (task failed)
- `Failed` \-\-\-\-\> `Working` (first launch after failed)
- `Working` \-\-\-\-\> `Success` (task success)
- `Success` \-\-\-\-\> `Not Found` (revoke)

If you equivalent `Not Found` to `Failed`, and ignore `revoke`, then:

`Failed` \<\-\-\-\> `Working` \-\-\-\-\> `Success`

## Nature
### About Task
1. A task is **launched** by passing a `Future<Output=Result<R, E>>` with unique `task_id`.
2. A task is `real_success` when return `Ok(R)`, and `real_failed` when return `Err(E)`.
3. Different future with **the same `task_id`** is considered **the same task**.
4. The same task **can only `real_success` once**, e.g. a purchase process would never succeed more then once by launching with unique process id as `task_id`.

### About Task State
1. If a task's state is `Success`, it must be `real_success`, i.e. $\text{Success}(id) \rightarrow \text{real\_success}(id)$.
2. If a task's state is `Failed`, it may be in any status, but mostly `real_failed`.
3. If a task's state is `Working`, it may be in any status, but mostly `real_working`.
4. If a task's state is `Not Found`, it may be in any status, but mostly `real_none`.

### About Task State Transition
1. Any task's state can be **queried** at any time.
2. The initial state of the task is `Not Found`.
3. Task's state won't change immediately after `launch` called. But if you query after `launch().await`, you will get changed result.
4. Always, when a task whose state is `Failed` or `NotFound` is launched, it will be `Working` at some future moment.
5. Always, when a task is `Working`, it would eventually be `Fail` or `Success`.
6. Always, when a task is `Success`, it would be `Success` forever.

# Other
Relationship between states and containers at [query_task_state](AsyncTasksRecoder::query_task_state).

Further propositions and proofs at [AsyncTasksRecoder](AsyncTasksRecoder).

Use [query_task_state_quick](AsyncTasksRecoder::query_task_state_quick) for less contention.


**For more usage, nature and proofs, please refer to Document.**

