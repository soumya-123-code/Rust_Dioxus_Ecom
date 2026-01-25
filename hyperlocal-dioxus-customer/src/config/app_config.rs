use crate::constants::ApiRoutes;

pub struct AppConfig;

impl AppConfig {
    pub fn api_base_url() -> String {
        ApiRoutes::BASE_URL.to_string()
    }
    
    pub fn is_demo_mode() -> bool {
        false // Set based on your needs
    }
}
