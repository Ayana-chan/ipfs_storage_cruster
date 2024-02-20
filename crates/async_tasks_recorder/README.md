# Introduction

A struct for recording execution status of async tasks with lock-free and async methods.

Can host `Future`s and query whether they are **not found**, **successful**, **failed**, or **running**.

- Depend on `tokio` with feature `rt`, so cannot use other async runtimes.
- Depend on [scc](https://crates.io/crates/scc) for lock-free and async `HashSet`.

Use this crate if:
- Easy to generate an **unique** `task_id` (not necessarily `String`) for a future (task).
- Tasks might fail, and then you want to run it again, while you don't want it to success more then once.
- Want to record and query all succeeded tasks and failed tasks.
- Want to handling every task in the same state (e.g. `success`).

# Example
[Here](https://github.com/Ayana-chan/ipfs_storage_cruster/tree/master/crates/async_tasks_recorder/examples).

# More Details
A recorder can only use one `task_id` type. The type of `task_id` should be:
- `Eq + Hash + Clone + Send + Sync + 'static`
- Cheap to clone (sometimes can use `Arc`).

And remember, you can add **anything** in the `Future` to achieve the functionality you want.
For example:
- Handle your `Result` in `Future`, and then return empty result `Result<(),()>`.
- Send a message to a one shot channel at the end of the `Future` to notify upper level that "This task is done".
  Don't forget to consider using `tokio::spawn` when the channel may not complete sending immediately.
- Set other callback functions.

> It is recommended to directly look at the source code (about 100 line) if there is any confusion.

**NOTE**: This crate use three `HashSet` to make it easy to handle all tasks in the same state.
But `scc::HashSet` have less contention in **single** access when it grows larger.
Therefore, if you don't need handling every tasks in the same state,
then just use `scc::HashMap` (`task_id` \-\> `task_status`) to build a simpler implementation,
which might have less contention and clone, but more expansive to iterator.

**For more usage, nature and proofs, please refer to Document.**

