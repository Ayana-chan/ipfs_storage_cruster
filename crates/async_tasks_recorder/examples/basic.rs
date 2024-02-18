use async_tasks_recorder::AsyncTasksRecoder;

struct SimulatedStream {}

struct UploadFileArgs {
    /// file stream
    stream: SimulatedStream,
    md5: String,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct UploadFileTaskId {
    /// file's md5
    md5: String,
    /// storage destination
    destination: String,
}

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
    upload_file(recorder.clone(),
                UploadFileArgs {
                    stream: SimulatedStream {},
                    md5: "d8q793wye1u3".to_string(),
                },
    ).await;
}

// APIs -----------

async fn upload_file(recorder: AsyncTasksRecoder<UploadFileTaskId>, args: UploadFileArgs) {
    println!("REQUEST: upload_file");
    let destination = "some place".to_string(); // decided by some algorithm
    let task_id = UploadFileTaskId {
        md5: args.md5,
        destination: destination.clone(),
    };
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

    recorder.launch(task_id, fut).await;
}

async fn check_upload_state() -> UploadTaskState {
    UploadTaskState::Success
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

