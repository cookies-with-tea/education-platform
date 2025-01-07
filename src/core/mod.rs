pub mod app;
pub mod db;
pub mod error;

pub use db::DatabaseConfig;
use std::sync::Arc;

use crate::user::routing as user_routing;
use crate::AppState;
use axum::Router;

pub fn create_routing(shared_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let app = Router::new().nest(
        "/api/v1/users",
        user_routing::create_router(shared_state.clone()),
    );
    app
}
