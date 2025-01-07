use std::collections::HashMap;

#[derive(serde_derive::Serialize)]
pub struct ApiResponse<T> {
    pub(crate) data: Option<T>,
    pub(crate) errors: Option<HashMap<String, Vec<String>>>,
    pub(crate) messages: Option<Vec<String>>,
}
