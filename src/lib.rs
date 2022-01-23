use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: String,
}

impl Task {
    pub fn is_completed(&self) -> bool {
        self.status == "done"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonApiResponse {
    pub data: Vec<Task>,
}
