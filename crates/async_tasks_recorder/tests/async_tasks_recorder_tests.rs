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
        test_basic(30, None, 200),
    );
}

#[test]
fn test_basic_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_basic(5, Some(1500), 100),
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
        test_random(30,
                    3, 400,
                    200, 13,
                    60),
    );
}

#[test]
fn test_random_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_random(8,
                    3, 600,
                    200, 13,
                    60),
    );
}

#[test]
fn test_stress_multi() {
    // do_async_test(
    //     RuntimeType::MultiThread,
    //     test_stress()
    // );
}

#[test]
fn test_stress_single() {
    // do_async_test(
    //     RuntimeType::CurrentThread,
    //     test_stress()
    // );
}

