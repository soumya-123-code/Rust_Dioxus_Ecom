use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub access_panel: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub user: Option<User>,
}

impl AuthState {
    pub fn new() -> Self {
        let token: Option<String> = LocalStorage::get("admin_token").ok();
        let user: Option<User> = LocalStorage::get("admin_user").ok();
        Self { token, user }
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    pub fn login(&mut self, token: String, user: User) {
        LocalStorage::set("admin_token", &token).ok();
        LocalStorage::set("admin_user", &user).ok();
        self.token = Some(token);
        self.user = Some(user);
    }

    pub fn logout(&mut self) {
        LocalStorage::delete("admin_token");
        LocalStorage::delete("admin_user");
        self.token = None;
        self.user = None;
    }

    pub fn get_token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

#[derive(Clone, Debug, Default)]
pub struct SidebarState {
    pub is_open: bool,
}

impl SidebarState {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    
    pub fn close(&mut self) {
        self.is_open = false;
    }
}
