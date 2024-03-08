#[allow(unused_imports)]
use tracing::{error, info};
use sea_orm::{Database, DbErr};
use sea_orm::prelude::DatabaseConnection;
use crate::app::errors;

static DATABASE_CONN_RETRY_INTERVAL_TIME_MS: u64 = 3000;

#[tracing::instrument(skip_all)]
pub async fn connect_db_until_success(db_url: &str) -> DatabaseConnection {
    loop {
        match Database::connect(db_url.to_string()).await {
            Ok(conn) => return conn,
            Err(e) => {
                error!("Failed to connect database: {:?}. Try again in {} ms. msg: {:?}",
                    db_url, DATABASE_CONN_RETRY_INTERVAL_TIME_MS, e);
                // wait
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    DATABASE_CONN_RETRY_INTERVAL_TIME_MS)).await;
            }
        }
    }
}

/// Convert and log database error.
pub fn handle_db_error(e: DbErr) -> errors::ResponseError {
    error!("Database error: {:?}", e);
    errors::DB_FAIL.clone_to_error()
}
