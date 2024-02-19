#![allow(dead_code, unused_variables)]

use std::sync::Arc;
use async_tasks_recorder::{AsyncTasksRecoder, TaskState};

struct SimulatedStream {}

struct UploadFileArgs {
    /// file stream
    stream: SimulatedStream,
    md5: String,
}

#[derive(Debug, Eq, PartialEq)]
enum UploadTaskState {
    Uploading,
    Failed,
    Success,
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(simulate_requests())
}

/// Simulate front-end request.
async fn simulate_requests() {
    println!("hello world!");
    let recorder = AsyncTasksRecoder::new();
    let fake_md5 = "d8q793wye1u3".to_string();

    println!("REQUEST: check_upload_state {}", fake_md5);
    let result = check_upload_state(
        recorder.clone(),
        fake_md5.to_string()
    ).await;
    assert_eq!(result, UploadTaskState::Failed);
    println!("RESPONSE: check_upload_state {}: {:?}", fake_md5, result);

    println!("REQUEST: upload_file {}", fake_md5);
    upload_file(recorder.clone(),
                UploadFileArgs {
                    stream: SimulatedStream {},
                    md5: fake_md5.clone(),
                },
    ).await;

    println!("REQUEST: check_upload_state {}", fake_md5);
    let result = check_upload_state(
        recorder.clone(),
        fake_md5.to_string()
    ).await;
    assert_eq!(result, UploadTaskState::Uploading);
    println!("RESPONSE: check_upload_state {}: {:?}", fake_md5, result);

    println!("WAIT");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("REQUEST: check_upload_state {}", fake_md5);
    let result = check_upload_state(
        recorder.clone(),
        fake_md5.to_string()
    ).await;
    assert_eq!(result, UploadTaskState::Success);
    println!("RESPONSE: check_upload_state {}: {:?}", fake_md5, result);
}

// APIs -----------

async fn upload_file(recorder: AsyncTasksRecoder<Arc<String>>, args: UploadFileArgs) {
    let destination = "some place".to_string(); // decided by some algorithm
    let fut = async move {
        println!("upload_to_destination start!");
        let res = upload_to_destination(args.stream, destination).await;
        match res {
            Ok(msg) => {
                println!("upload_to_destination finish! msg: {}", msg);
                Ok(())
            }
            Err(msg) => {
                println!("upload_to_destination error! msg: {}", msg);
                Err(())
            }
        }
    };

    // launch `Arc<String>` and `Future`
    recorder.launch(args.md5.into(), fut).await;
}

async fn check_upload_state(recorder: AsyncTasksRecoder<Arc<String>>, arg_md5: String) -> UploadTaskState {
    let arg_md5 = arg_md5.into();
    let res = recorder.query_task_state(&arg_md5).await;

    match res {
        TaskState::Success => UploadTaskState::Success,
        TaskState::Failed => UploadTaskState::Failed,
        TaskState::Working => UploadTaskState::Uploading,
    }
}

// other functions ------------

async fn upload_to_destination(stream: SimulatedStream, destination: String) -> Result<String, String> {
    // simulate uploading stream to destination
    std::thread::sleep(std::time::Duration::from_millis(50));
    let res = "no problem".to_string(); // result of upload
    // async large callback
    tokio::spawn(large_callback());
    Ok(res)
}

async fn large_callback() {
    println!("large_callback");
    std::thread::sleep(std::time::Duration::from_millis(10));
    println!("large_callback finish");
}

