use async_tasks_recorder::*;
pub use tools::do_async_test;

mod task;
mod tools;
mod launch;
mod check;

pub static DEFAULT_CHECK_INTERVAL_MS: u64 = 5;
pub static DEFAULT_CHECK_TIMEOUT_MS: u128 = 1000;

#[allow(dead_code)]
pub enum RuntimeType {
    CurrentThread,
    CustomThread(usize),
    MultiThread,
}

pub async fn test_once_core() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, task::success_task(13, id.clone())).await;
    check::check_success(&manager, &id, None, None, 60).await;
}

pub async fn test_once_fail_core() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, task::fail_task(13, id.clone())).await;
    check::check_success(&manager, &id, None, Some(100), 60).await;
}

pub async fn test_basic_core(task_num: usize, check_time_out: Option<u128>, check_suffix_query_time: u128) {
    let manager = AsyncTasksRecoder::new();

    let task_id_vec = tools::generate_task_id_vec(task_num);
    let task_id_vec = task_id_vec.into();

    launch::launch_vec(&manager, &task_id_vec, 0..60, 100).await;
    check::check_success_vec(&manager, &task_id_vec, None, check_time_out, check_suffix_query_time).await;
}

pub async fn test_once_redo_core() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, task::fail_task(13, id.clone())).await;
    check::check_success_auto_redo(&manager, &id, None, Some(100), 200,
                                   13, 100).await;
}

pub async fn test_random_core(task_num: usize,
                              check_interval_ms: u64, check_time_out_ms: u128,
                              suffix_query_time: u128, redo_task_latency: u64,
                              task_success_probability: u8) {
    let manager = AsyncTasksRecoder::new();

    let task_id_vec = tools::generate_task_id_vec(task_num);
    let task_id_vec = task_id_vec.into();

    launch::launch_vec(&manager, &task_id_vec, 2..15, task_success_probability).await;
    check::check_success_vec_auto_redo(&manager, &task_id_vec,
                                       Some(check_interval_ms), Some(check_time_out_ms),
                                       suffix_query_time, redo_task_latency,
                                       task_success_probability).await;
}
