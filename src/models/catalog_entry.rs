use serde::Deserialize;
use serde::Serialize;

use crate::models::education::CourseOnClient;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CatalogEntry {
    pub track_id: String,
    pub track_name: String,
    pub course: CourseOnClient,
}

impl CatalogEntry {
    pub fn new(track_id: String, track_name: String, course: CourseOnClient) -> Self {
        Self {
            track_id,
            track_name,
            course,
        }
    }
}
