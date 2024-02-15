use std::ops::RangeBounds;
use std::sync::Arc;
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

pub async fn test_once() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, task::success_task(13, id.clone())).await;
    check::check_success(&manager, &id, None, None, 60).await;
}

pub async fn test_once_fail() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, task::fail_task(13, id.clone())).await;
    check::check_success(&manager, &id, None, Some(100), 60).await;
}

pub async fn test_basic(task_num: usize, task_latency_range: impl RangeBounds<u64> + Clone,
                        check_time_out: Option<u128>, check_suffix_query_time: u128) {
    let manager = AsyncTasksRecoder::new();

    let task_id_vec = tools::generate_task_id_vec(task_num);
    let task_id_vec = task_id_vec.into();

    launch::launch_vec(&manager, &task_id_vec, task_latency_range, 100).await;
    check::check_success_vec(&manager, &task_id_vec, None, check_time_out, check_suffix_query_time).await;
}

pub async fn test_once_redo() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = tools::get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, task::fail_task(13, id.clone())).await;
    check::check_success_auto_redo(&manager, &id, None, Some(100), 200,
                                   13, 100).await;
}

pub async fn test_random(task_num: usize, task_latency_range: impl RangeBounds<u64> + Clone,
                         check_interval_ms: u64, check_time_out_ms: u128,
                         suffix_query_time: u128, redo_task_latency: u64,
                         task_success_probability: u8) {
    let manager = AsyncTasksRecoder::new();

    let task_id_vec = tools::generate_task_id_vec(task_num);
    let task_id_vec = task_id_vec.into();

    launch::launch_vec(&manager, &task_id_vec, task_latency_range, task_success_probability).await;
    check::check_success_vec_auto_redo(&manager, &task_id_vec,
                                       Some(check_interval_ms), Some(check_time_out_ms),
                                       suffix_query_time, redo_task_latency,
                                       task_success_probability).await;
}

pub async fn test_interleave(group_num: usize, group_size_min: usize, group_size_max: usize,
                             task_latency_range: impl RangeBounds<u64> + Clone,
                             check_interval_ms: u64, check_time_out_ms: u128,
                             suffix_query_time: u128, redo_task_latency: u64,
                             task_success_probability: u8) {
    let group_size_vec = tools::get_arithmetic_sequence(group_num, group_size_min, group_size_max);
    println!("group_size_vec: {:?}", group_size_vec);
    let mut group_task_id_vec: Vec<Arc<Vec<String>>> = vec![];
    group_task_id_vec.resize(group_num, Arc::new(vec![]));
    for i in 0..group_num {
        group_task_id_vec[i] = tools::generate_task_id_vec(group_size_vec[i]).into();
    }

    // decide the sequence of launch and check (in units of groups)
    let mut working_seq: Vec<usize> = (0..group_num * 2).collect();
    fastrand::shuffle(&mut working_seq);
    println!("working_seq: {:?}", working_seq);

    let manager = AsyncTasksRecoder::new();
    let mut check_join_set = tokio::task::JoinSet::new();

    for cur in working_seq {
        let group_id = cur / 2;
        if cur % 2 == 0 {
            // launch
            println!("launch group {}, group size: {}, group task_id vec: {:?}", group_id, group_size_vec[group_id], group_task_id_vec[group_id]);
            launch::launch_vec(&manager, &group_task_id_vec[group_id],
                               task_latency_range.clone(), task_success_probability).await;
        } else {
            // check
            println!("check group {}, group size: {}, group task_id vec: {:?}", group_id, group_size_vec[group_id], group_task_id_vec[group_id]);
            let task_id_vec = group_task_id_vec[group_id].clone();
            let manager = manager.clone();
            let check_fut = async move {
                check::check_success_vec_auto_redo(&manager, &task_id_vec,
                                                   Some(check_interval_ms), Some(check_time_out_ms),
                                                   suffix_query_time, redo_task_latency,
                                                   task_success_probability).await;
            };
            check_join_set.spawn(check_fut);
        }
    }

    while let Some(res) = check_join_set.join_next().await {
        if let Err(e) = res {
            if e.is_panic() {
                std::panic::resume_unwind(e.into_panic());
            }
        }
    }
}

