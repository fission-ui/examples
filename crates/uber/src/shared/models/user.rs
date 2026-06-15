use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub profile_image_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            email: String::new(),
            phone: String::new(),
            profile_image_url: None,
            created_at: chrono::Utc::now(),
        }
    }
}
