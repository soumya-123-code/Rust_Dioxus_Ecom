use dioxus::prelude::*;
use crate::types::UserData;
use crate::services::storage::StorageService;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user: Option<UserData>,
    pub token: Option<String>,
    pub is_authenticated: bool,
}

impl Default for AuthContext {
    fn default() -> Self {
        let token = StorageService::get_token().ok().flatten();
        let user = StorageService::get_user_data::<UserData>().ok().flatten();
        
        Self {
            user,
            token: token.clone(),
            is_authenticated: token.is_some(),
        }
    }
}

pub fn use_auth() -> Signal<AuthContext> {
    use_context::<Signal<AuthContext>>()
}
