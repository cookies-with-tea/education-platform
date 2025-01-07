use crate::core::dto::ApiResponse;
use crate::core::error::format_error;
use crate::event::dto::Event;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::query_as;
use std::sync::Arc;
// async fn create_event()

async fn get_all(
    state: State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Event>>>, (StatusCode, Json<ApiResponse<()>>)> {
    match query_as::<_, Event>("SELECT * FROM event")
        .fetch_all(&state.pool)
        .await
    {
        Ok(events) => {
            let response = ApiResponse {
                data: Some(events),
                errors: None,
                messages: None,
            };
            Ok(Json(response))
        }
        Err(_) => {
            let errors = format_error(
                "database",
                vec![
                    "Database connection failed".to_string(),
                    "Check your query or database status".to_string(),
                ],
            );
            let response = ApiResponse {
                data: None,
                errors: Some(errors),
                messages: Some(vec!["An error occurred while fetching users.".to_string()]),
            };
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}

pub fn routing(shared_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let mut app = Router::new();

    app = app.route("/", get(get_all));

    app
}
