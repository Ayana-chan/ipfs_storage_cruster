#[allow(unused_imports)]
use async_tasks_recorder::*;
use async_tasks_recorder_tests_repo::*;

mod async_tasks_recorder_tests_repo;

#[test]
fn test_once_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_once(),
    );
}

#[test]
fn test_once_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_once(),
    );
}

#[test]
#[should_panic(expected = "Timeout before success")]
fn test_once_fail_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_once_fail(),
    );
}

#[test]
#[should_panic(expected = "Timeout before success")]
fn test_once_fail_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_once_fail(),
    );
}

#[test]
fn test_basic_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_basic(30, 0..60, None, 200),
    );
}

#[test]
fn test_basic_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_basic(5, 0..60, Some(1500), 100),
    );
}

#[test]
fn test_redo_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_once_redo(),
    );
}

#[test]
fn test_redo_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_once_redo(),
    );
}

#[test]
fn test_random_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_random(30, 2..15,
                    3, 400,
                    200, 13,
                    60),
    );
}

#[test]
fn test_random_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_random(8, 2..15,
                    3, 600,
                    200, 13,
                    60),
    );
}

#[test]
fn test_interleave_multi() {
    do_async_test(
        RuntimeType::MultiThread,
        test_interleave(30, 2, 5,
                        2..15,
                        3, 600,
                        200, 13,
                        60),
    );
}

#[test]
fn test_interleave_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_interleave(5, 2, 5,
                        2..15,
                        3, 600,
                        200, 13,
                        60),
    );
}

#[test]
fn test_stress_large_group_num() {
    do_async_test(
        RuntimeType::MultiThread,
        test_interleave(300, 2, 5,
                        2..15,
                        1, 3000,
                        500, 13,
                        60),
    );
}

#[test]
fn test_stress_large_group_size_range() {
    do_async_test(
        RuntimeType::MultiThread,
        test_interleave(50, 1, 50,
                        2..15,
                        1, 3000,
                        500, 13,
                        60),
    );
}

#[test]
fn test_stress_large_task_latency_range() {
    do_async_test(
        RuntimeType::MultiThread,
        test_interleave(50, 10, 20,
                        0..50,
                        1, 3000,
                        500, 13,
                        60),
    );
}
