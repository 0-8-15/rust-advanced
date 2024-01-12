use serde::{Deserialize, Serialize};
 
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub category: Category,
    #[serde(rename = "photoUrls")]
    pub photo_urls: Vec<String>,
    pub tags: Vec<Tag>,
    pub status: String,
}
 
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}
 
impl Pet {
    pub fn new(id: i64, name: String) -> Self {
        Self {
            id,
            name,
            category: Category {
                id: 0,
                name: String::new(),
            },
            photo_urls: vec![],
            tags: vec![],
            status: "available".to_string(),
        }
    }
}
