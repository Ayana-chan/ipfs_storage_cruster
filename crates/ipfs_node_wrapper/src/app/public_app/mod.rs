use axum::{
    Router,
    routing::get,
};

mod handlers;

use handlers::*;

pub fn generate_public_app() -> Router {
    let app = Router::new()
        .route("/", get(get_file));

    Router::new().nest("/api", app)
}

