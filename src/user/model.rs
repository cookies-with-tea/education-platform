use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateUserDTO {
    pub(crate) first_name: String,
}

// #[derive(Deserialize, Debug)]
// pub struct UpdateUserDTO {
//     pub(crate) first_name: Option<String>,
//     pub(crate) last_name: Option<String>,
// }

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    uuid: Uuid,
    id: i32,
    first_name: String,
    second_name: String,
    last_name: String,
    phone: String,
    birth_date: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
