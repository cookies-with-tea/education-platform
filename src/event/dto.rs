use chrono::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Event {
    uuid: Uuid,
    id: i32,
    title: String,
    description: String,
    #[serde(rename = "event_type")]
    event_type: EventType,
    date: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(type_name = "event_type_enum", rename_all = "lowercase")]
#[serde(rename_all = "kebab-case")]
pub enum EventType {
    Meet,
    Webinar,
    Announcement,
}