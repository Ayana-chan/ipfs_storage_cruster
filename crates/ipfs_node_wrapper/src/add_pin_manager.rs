use std::sync::Arc;
use crate::ipfs_client::IpfsClient;

#[derive(Default, Debug)]
pub struct AddPinManager {
    /// cid -> state
    working_tasks: scc::HashSet<String>,
    success_tasks: scc::HashSet<String>,
    failed_tasks: scc::HashSet<String>,
}

impl AddPinManager {
    pub fn new() -> Self {
        AddPinManager {
            working_tasks: scc::HashSet::new(),
            success_tasks: scc::HashSet::new(),
            failed_tasks: scc::HashSet::new(),
        }
    }

    pub async fn launch(self: &Arc<Self>, ipfs_client: &IpfsClient, cid: &str, name: Option<&str>) {
        // check pin status
        if self.success_tasks.contains_async(cid).await {
            return;
        }
        let was_failed = self.failed_tasks.contains_async(cid).await;
        // TODO 这里查一下ipfs里面有没有pin，但已failed的时候也许不用查。要优化一下working_tasks的查询位置来防止浪费查询时间
        let res = self.working_tasks.insert_async(cid.to_string()).await;
        if res.is_err() {
            return;
        }
        if was_failed {
            self.failed_tasks.remove(cid);
        }

        // adjust args to meet borrow checker
        let self_arc = self.clone();
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
            if let Ok(_success_res) = add_pin_res {
                let _ = self_arc.success_tasks.insert_async(cid.clone()).await;
                self_arc.working_tasks.remove_async(&cid).await;
            } else {
                let _ = self_arc.failed_tasks.insert_async(cid.clone()).await;
                self_arc.working_tasks.remove_async(&cid).await;
            }
        });
    }
}

