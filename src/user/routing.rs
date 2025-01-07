use crate::user::handlers::{create_user, get_users, get_user};
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn create_router(shared_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let mut app = Router::new();

    app = app
        .route(
            "/",
            post({
                let shared_state = shared_state.clone();
                move |body| create_user(body, shared_state.clone())
            }),
        )
        .route("/", get(get_users))
        .route("/{uuid}", get({
            let shared_state = shared_state.clone();
            move |uuid| get_user(uuid, shared_state.clone())
        }));

    app
}
