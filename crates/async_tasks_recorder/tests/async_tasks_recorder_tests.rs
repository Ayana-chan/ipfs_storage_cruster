use async_tasks_recorder::*;
use std::future::Future;
use std::ops::RangeBounds;
use std::sync::Arc;

static DEFAULT_CHECK_INTERVAL_MS: u64 = 5;
static DEFAULT_CHECK_TIMEOUT_MS: u128 = 1000;

#[allow(dead_code)]
enum RuntimeType {
    CurrentThread,
    CustomThread(usize),
    MultiThread,
}

#[test]
fn test_once() {
    do_async_test(
        RuntimeType::MultiThread,
        test_once_core(),
    );
}

#[test]
fn test_once_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_once_core(),
    );
}

#[test]
#[should_panic(expected = "Timeout before success")]
fn test_once_fail() {
    do_async_test(
        RuntimeType::MultiThread,
        test_once_fail_core(),
    );
}

#[test]
#[should_panic(expected = "Timeout before success")]
fn test_once_fail_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_once_fail_core(),
    );
}

#[test]
fn test_basic() {
    do_async_test(
        RuntimeType::MultiThread,
        test_basic_core(30),
    );
}

#[test]
fn test_basic_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_basic_core(5),
    );
}

#[test]
fn test_redo() {
    do_async_test(
        RuntimeType::MultiThread,
        test_once_redo_core(),
    );
}

#[test]
fn test_redo_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_once_redo_core(),
    );
}

#[test]
fn test_random() {
    do_async_test(
        RuntimeType::MultiThread,
        test_random_core(30,
                         3, 400,
                         0, 13,
                         60),
    );
}

#[test]
fn test_random_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_random_core(8,
                         3, 600,
                         0, 13,
                         60),
    );
}

// core functions ---------------------------------------------------------------

async fn test_once_core() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, success_task(13)).await;
    check_success(&manager, &id, None, None).await;
}

async fn test_once_fail_core() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, fail_task(13)).await;
    check_success(&manager, &id, None, Some(100)).await;
}

async fn test_basic_core(task_num: usize) {
    let manager = AsyncTasksRecoder::new();

    let task_id_vec = generate_task_id_vec(task_num);
    let task_id_vec = task_id_vec.into();

    launch_vec(&manager, &task_id_vec, 0..60, 100).await;
    check_success_vec(&manager, &task_id_vec, None, None).await;
}

async fn test_once_redo_core() {
    let manager = AsyncTasksRecoder::new();
    let mut task_id_generator = get_task_id_generator();

    let id = task_id_generator();
    manager.launch(&id, fail_task(13)).await;
    check_success_auto_redo(&manager, &id, None, Some(100),
                            0, 13, 100).await;
}

async fn test_random_core(task_num: usize,
                          check_interval_ms: u64, check_time_out_ms: u128,
                          redo_delay_ms: u64, redo_task_latency: u64,
                          task_success_probability: u8) {
    let manager = AsyncTasksRecoder::new();

    let task_id_vec = generate_task_id_vec(task_num);
    let task_id_vec = task_id_vec.into();

    launch_vec(&manager, &task_id_vec, 2..15, task_success_probability).await;
    check_success_vec_auto_redo(&manager, &task_id_vec,
                                Some(check_interval_ms), Some(check_time_out_ms),
                                redo_delay_ms, redo_task_latency,
                                task_success_probability).await;
}

// tools ------------------------------------------------------------------------

fn do_async_test<Fut>(runtime_type: RuntimeType, test_future: Fut)
    where Fut: Future {
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

    runtime.block_on(test_future);
}

/// Launch success task by `task_id_vec`.
/// The latency of each task is randomly selected within `latency_range`.
async fn launch_vec<Range>(manager: &AsyncTasksRecoder, task_id_vec: &Arc<Vec<String>>,
                           latency_range: Range, task_success_probability: u8)
    where Range: RangeBounds<u64> + Clone {
    let task_num = task_id_vec.len();
    let shuffled_map = get_shuffled_index_map(task_num);
    for i in 0..task_num {
        let manager_backup = manager.clone();
        let task_id_vec = task_id_vec.clone();
        let mapped_index = shuffled_map[i];
        let latency = fastrand::u64(latency_range.clone());
        // println!("spawn launch: {} latency: {}", mapped_index, latency);
        let fut = async move {
            manager_backup.launch(&task_id_vec[mapped_index],
                                  random_task(latency, task_success_probability)).await;
        };
        tokio::spawn(fut);
    }
}

/// If `redo` is `Some(t)`, it will be redo after a delay after the task is found to have failed.
/// Check per `interval_ms`. \
/// Err when the time consumption reaches `timeout_ms`.
async fn check_success(manager: &AsyncTasksRecoder, task_id: &str,
                       interval_ms: Option<u64>, timeout_ms: Option<u128>) {
    let interval_ms = interval_ms.unwrap_or(DEFAULT_CHECK_INTERVAL_MS.clone());
    let timeout_ms = timeout_ms.unwrap_or(DEFAULT_CHECK_TIMEOUT_MS.clone());
    let start_time = std::time::Instant::now();

    let mut task_status;

    loop {
        // println!("check {}, interval: {}", task_id, interval_ms);
        // timeout
        if start_time.elapsed().as_millis() >= timeout_ms {
            panic!("Timeout before success. task_id: {:?}", task_id);
        }

        task_status = manager.get_task_status(task_id).await;
        if task_status == TaskStatus::Success {
            return;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms.clone())).await;
    }
}

/// A task will be redo after a delay (`redo_delay_ms`) after it is found to have failed.
/// Check per `interval_ms`. \
/// Err when the time consumption reaches `timeout_ms`.
async fn check_success_auto_redo(manager: &AsyncTasksRecoder, task_id: &str,
                                 interval_ms: Option<u64>, timeout_ms: Option<u128>,
                                 redo_delay_ms: u64, redo_task_latency: u64,
                                 redo_task_success_probability: u8) {
    let interval_ms = interval_ms.unwrap_or(DEFAULT_CHECK_INTERVAL_MS.clone());
    let timeout_ms = timeout_ms.unwrap_or(DEFAULT_CHECK_TIMEOUT_MS.clone());
    let start_time = std::time::Instant::now();

    let mut task_status;

    loop {
        // println!("check {}, interval: {}", task_id, interval_ms);
        // timeout
        if start_time.elapsed().as_millis() >= timeout_ms {
            panic!("Timeout before success. task_id: {:?}", task_id);
        }

        task_status = manager.get_task_status(task_id).await;
        match task_status {
            TaskStatus::Success => {
                println!("success {}, used time: {}", task_id, start_time.elapsed().as_millis());
                return;
            }
            TaskStatus::Failed => {
                // redo
                println!("redo {}", task_id);
                tokio::time::sleep(tokio::time::Duration::from_millis(redo_delay_ms)).await;
                let task = random_task(redo_task_latency, redo_task_success_probability);
                manager.launch(task_id, task).await;
            }
            TaskStatus::Working => {}
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms.clone())).await;
    }
}

/// Check all task_id in a vec randomly and parallelly.
/// Return after all checks finish.
async fn check_success_vec(manager: &AsyncTasksRecoder, task_id_vec: &Arc<Vec<String>>,
                           interval_ms: Option<u64>, timeout_ms: Option<u128>) {
    let task_num = task_id_vec.len();
    let shuffled_map = get_shuffled_index_map(task_num);
    let mut join_set = tokio::task::JoinSet::new();
    for i in 0..task_num {
        let manager_backup = manager.clone();
        let task_id_vec = task_id_vec.clone();
        let mapped_index = shuffled_map[i];
        // println!("spawn check {}", mapped_index);
        let fut = async move {
            check_success(&manager_backup, &task_id_vec[mapped_index],
                          interval_ms.clone(), timeout_ms.clone()).await;
        };
        join_set.spawn(fut);
    }

    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            if e.is_panic() {
                std::panic::resume_unwind(e.into_panic());
            }
        }
    }
}

/// Check all task_id in a vec randomly and parallelly.
/// Return after all checks finish.
async fn check_success_vec_auto_redo(manager: &AsyncTasksRecoder, task_id_vec: &Arc<Vec<String>>,
                                     interval_ms: Option<u64>, timeout_ms: Option<u128>,
                                     redo_delay_ms: u64, redo_task_latency: u64,
                                     redo_task_success_probability: u8) {
    let task_num = task_id_vec.len();
    let shuffled_map = get_shuffled_index_map(task_num);
    let mut join_set = tokio::task::JoinSet::new();
    for i in 0..task_num {
        let manager_backup = manager.clone();
        let task_id_vec = task_id_vec.clone();
        let mapped_index = shuffled_map[i];
        // println!("spawn check {}", mapped_index);
        let fut = async move {
            check_success_auto_redo(&manager_backup, &task_id_vec[mapped_index],
                                    interval_ms.clone(), timeout_ms.clone(),
                                    redo_delay_ms, redo_task_latency,
                                    redo_task_success_probability).await;
        };
        join_set.spawn(fut);
    }

    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            if e.is_panic() {
                std::panic::resume_unwind(e.into_panic());
            }
        }
    }
}

/// return a closure to generate incremental `task_id` string.
fn get_task_id_generator() -> impl FnMut() -> String {
    let rand_str: String = std::iter::repeat_with(fastrand::alphabetic).take(6).collect();
    let mut count: usize = 0;
    move || {
        let ans = format!("task_id_{}_{:06}", rand_str, count);
        count += 1;
        ans
    }
}

/// generate `task_id` vec with a new generator from `get_task_id_generator`
fn generate_task_id_vec(task_num: usize) -> Vec<String> {
    let mut task_id_generator = get_task_id_generator();
    let mut task_id_vec: Vec<String> = Vec::new();
    task_id_vec.resize(task_num, Default::default());
    for i in 0..task_num {
        task_id_vec[i] = task_id_generator();
    }
    task_id_vec
}

/// get a shuffled `vec[0, 1, 2, ...]` to map index
fn get_shuffled_index_map(length: usize) -> Vec<usize> {
    let mut map: Vec<usize> = (0..length).collect();
    fastrand::shuffle(&mut map);
    map
}

// TODO 检验被调用次数
/// A task always return ok. \
/// Latency is caused by `std::thread::sleep`, which won't stop when panic in main thread occurs.
async fn success_task(latency_ms: u64) -> Result<(), ()> {
    // must use `sleep` of std
    std::thread::sleep(std::time::Duration::from_millis(latency_ms));
    println!("---->finish, task latency: {}", latency_ms);
    Ok(())
}

/// A task always return err.
async fn fail_task(latency_ms: u64) -> Result<(), ()> {
    std::thread::sleep(std::time::Duration::from_millis(latency_ms));
    println!("---->fail, task latency: {}", latency_ms);
    Err(())
}

/// A task possibly return err. \
/// `success_probability`: The percentage probability of success. Supposed to be \[0, 100\].
async fn random_task(latency_ms: u64, success_probability: u8) -> Result<(), ()> {
    std::thread::sleep(std::time::Duration::from_millis(latency_ms));
    let rand_point = fastrand::u8(0..100);
    if rand_point < success_probability {
        println!("---->rand Ok, point: {} < {}, task latency: {}", rand_point, success_probability, latency_ms);
        Ok(())
    } else {
        println!("rand Err, point: {} >= {}, task latency: {}", rand_point, success_probability, latency_ms);
        Err(())
    }
}




