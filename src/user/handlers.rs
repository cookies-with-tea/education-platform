use std::collections::HashMap;
use crate::core::error::internal_error;
use crate::user::model::{CreateUserDTO, User};
use crate::AppState;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract, Json};
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde_derive::Serialize)]
pub struct ApiResponse<T> {
    data: Option<T>,
    errors: Option<HashMap<String, Vec<String>>>,
    messages: Option<Vec<String>>,
}

fn format_error(key: &str, messages: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();

    errors.insert(key.to_string(), messages);

    errors
}

pub(crate) async fn create_user(
    Json(payload): Json<CreateUserDTO>,
    state: Arc<AppState>,
) -> StatusCode {
    let _ = sqlx::query("INSERT INTO education_user (first_name) VALUES ($1)")
        .bind(&payload.first_name)
        .execute(&state.pool)
        .await
        .map_err(internal_error)
        .unwrap();

    StatusCode::CREATED
}

pub(crate) async fn get_users(
    state: extract::State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<User>>>, (StatusCode, Json<ApiResponse<()>>)> {
    match sqlx::query_as::<_, User>("SELECT * FROM education_user")
        .fetch_all(&state.pool)
        .await
    {
        Ok(users) => {
            let response = ApiResponse {
                data: Some(users),
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

pub(crate) async fn get_user(
    Path(uuid): Path<Uuid>,
    state: Arc<AppState>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM education_user WHERE uuid = $1")
        .bind(uuid)
        .fetch_optional(&state.pool)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

    match result {
        Some(user) => {
            Ok(Json(user))
        }
        None => Err((StatusCode::NOT_FOUND, "Пользователь не найден.".to_string())),
    }
}

/*async fn delete_user(Path(id): Path<i32>, state: Arc<AppState>) -> StatusCode {
    let _ = sqlx::query("DELETE FROM education_user WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(internal_error)
        .unwrap();

    StatusCode::OK
}*/