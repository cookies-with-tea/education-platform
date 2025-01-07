use chrono::NaiveDateTime;
use serde_derive::{Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Event {
    uuid: Uuid,
    id: i32,
    title: String,
    description: String,
    event_type: String,
    date: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}