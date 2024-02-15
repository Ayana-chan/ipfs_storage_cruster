#[allow(unused_imports)]
use async_tasks_recorder::*;
use async_tasks_recorder_tests_repo::*;

mod async_tasks_recorder_tests_repo;

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
        test_basic_core(30, None, 200),
    );
}

#[test]
fn test_basic_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_basic_core(5, Some(1500), 100),
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
                         200, 13,
                         60),
    );
}

#[test]
fn test_random_single() {
    do_async_test(
        RuntimeType::CurrentThread,
        test_random_core(8,
                         3, 600,
                         200, 13,
                         60),
    );
}

#[test]
fn test_stress() {}

