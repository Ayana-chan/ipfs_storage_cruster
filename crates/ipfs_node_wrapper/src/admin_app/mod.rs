use axum::{
    Router,
    routing::get,
};

mod handlers;

pub fn generate_admin_app() -> Router {
    let app = Router::new()
        .route("/", get(|| async { "Soyorin Love!" }));

    Router::new().nest("/api", app)
}
