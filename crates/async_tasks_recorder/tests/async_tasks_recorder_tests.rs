use async_tasks_recorder::*;
use std::future::Future;

static DEFAULT_CHECK_INTERVAL_MS: u64 = 5;
static DEFAULT_CHECK_TIMEOUT_MS: u128 = 1000;

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

// #[test]
// fn test_basic() {}
//
// #[test]
// fn test_random() {}

// core functions ---------------------------------------------------------------

async fn test_once_core() {
    let manager = AsyncTasksRecoder::new();
    manager.launch("t1", empty_task()).await;
    check_success(&manager, "t1", None, None).await;
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

async fn check_success_once(manager: &AsyncTasksRecoder, cid: &str) -> bool {
    manager.get_task_status(cid).await == TaskStatus::Pinned
}

/// Err when timeout
async fn check_success(manager: &AsyncTasksRecoder, cid: &str, interval_ms: Option<u64>, timeout_ms: Option<u128>) {
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

// TODO 定制延迟；定制概率（至少不能是50%）
/// A task always return ok.
async fn empty_task() -> Result<(), ()> {
    Ok(())
}

/// A task possibly return err.
async fn empty_random_task() -> Result<(), ()> {
    if fastrand::bool() {
        Ok(())
    } else {
        Err(())
    }
}
