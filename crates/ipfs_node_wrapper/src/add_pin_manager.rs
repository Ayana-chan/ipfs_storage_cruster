use std::sync::Arc;
use crate::ipfs_client::IpfsClient;

#[derive(Default, Debug)]
struct TaskManager {
    /// cid -> state
    working_tasks: scc::HashSet<String>,
    success_tasks: scc::HashSet<String>,
    failed_tasks: scc::HashSet<String>,
}

#[derive(Default, Debug)]
pub struct AddPinManager {
    /// cid -> state
    task_manager: Arc<TaskManager>,
}

impl AddPinManager {
    pub fn new() -> Self {
        AddPinManager {
            task_manager: TaskManager {
                working_tasks: scc::HashSet::new(),
                success_tasks: scc::HashSet::new(),
                failed_tasks: scc::HashSet::new(),
            }.into()
        }
    }

    pub async fn launch(&self, ipfs_client: &IpfsClient, cid: &str, name: Option<&str>) {
        // check pin status
        if self.task_manager.success_tasks.contains_async(cid).await {
            return;
        }
        let was_failed = self.task_manager.failed_tasks.contains_async(cid).await;
        // TODO 这里查一下ipfs里面有没有pin，但已failed的时候也许不用查。要优化一下working_tasks的查询位置来防止浪费查询时间
        let res = self.task_manager.working_tasks.insert_async(cid.to_string()).await;
        if res.is_err() {
            return;
        }
        if was_failed {
            self.task_manager.failed_tasks.remove(cid);
        }

        // adjust args
        let task_manager = self.task_manager.clone();
        let ipfs_client = ipfs_client.clone();
        let cid = cid.to_string();
        let name = name.map(String::from);

        // start
        let _task = tokio::spawn(async move {
            let add_pin_res = ipfs_client
                .add_pin_recursive(
                    &cid,
                    name.as_deref(),
                ).await;
            // Guarantee any launched cid can be found in one of the sets.
            // But it causes a copy of cid.
            // TODO 优化这里的string clone
            if let Ok(_success_res) = add_pin_res {
                let _ = task_manager.success_tasks.insert_async(cid.clone()).await;
                task_manager.working_tasks.remove_async(&cid).await;
            } else {
                let _ = task_manager.failed_tasks.insert_async(cid.clone()).await;
                task_manager.working_tasks.remove_async(&cid).await;
            }
        });
    }
}

