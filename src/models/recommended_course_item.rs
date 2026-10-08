use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RecommendedCourseItem {
    pub id: String,
    pub lesson_count: String,
    pub level: String,
    pub resource_title: String,
    pub resource_by: String,
    pub action_label: String,
    pub image_url: Option<String>,
}

impl RecommendedCourseItem {
    pub fn new(
        id: String,
        lesson_count: String,
        level: String,
        resource_title: String,
        resource_by: String,
        action_label: String,
        image_url: Option<String>,
    ) -> Self {
        Self {
            id,
            lesson_count,
            level,
            resource_title,
            resource_by,
            action_label,
            image_url,
        }
    }
}
