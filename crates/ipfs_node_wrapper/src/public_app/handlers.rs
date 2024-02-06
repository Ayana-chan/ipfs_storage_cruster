use tracing::info;

#[tracing::instrument]
pub async fn get_file(){
    info!("Get File");
}
