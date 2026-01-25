use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn success_message(msg: &str) -> Self {
        Self {
            success: true,
            message: Some(msg.to_string()),
            data: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message: Some(message),
            data: None,
        }
    }
}

// Implement for () for cases with no data
impl ApiResponse<()> {
    pub fn empty_success() -> Self {
        Self {
            success: true,
            message: None,
            data: None,
        }
    }
}
