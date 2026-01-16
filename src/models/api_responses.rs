use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T>{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn data(data: T) -> Self {
        Self {data: Some(data), error: None}
    }

    pub fn error(error: String) -> Self {
        Self {data: None, error: Some(error)}
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MosqueApiResponse {
    pub id: String,
    pub location: (f64, f64),
    pub name: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
}

