#[allow(unused_imports)]
use tracing::{trace, debug, info, error};
use crate::imports::dao_imports::*;
use crate::app::errors;

#[tracing::instrument(skip_all)]
pub async fn connect_db_until_success(db_url: &str, interval_time_ms: u64) -> DatabaseConnection {
    loop {
        match Database::connect(db_url.to_string()).await {
            Ok(conn) => return conn,
            Err(e) => {
                error!("Failed to connect database: {:?}. Try again in {} ms. msg: {:?}",
                    db_url, interval_time_ms, e);
                // wait
                tokio::time::sleep(tokio::time::Duration::from_millis(interval_time_ms)).await;
            }
        }
    }
}

/// Convert and log database error.
pub fn handle_db_error(e: DbErr) -> errors::ResponseError {
    error!("Database error: {:?}", e);
    errors::DB_FAIL.clone_to_error()
}

