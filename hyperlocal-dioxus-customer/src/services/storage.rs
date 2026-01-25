use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const TOKEN_KEY: &str = "auth_token";
const USER_DATA_KEY: &str = "user_data";

pub struct StorageService;

impl StorageService {
    pub fn save_token(token: &str) -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::set(TOKEN_KEY, token)
    }

    pub fn get_token() -> Result<Option<String>, gloo_storage::errors::StorageError> {
        LocalStorage::get(TOKEN_KEY)
    }

    pub fn remove_token() -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::delete(TOKEN_KEY);
        Ok(())
    }

    pub fn save_user_data<T: Serialize>(data: &T) -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::set(USER_DATA_KEY, data)
    }

    pub fn get_user_data<T: for<'de> Deserialize<'de>>() -> Result<Option<T>, gloo_storage::errors::StorageError> {
        LocalStorage::get(USER_DATA_KEY)
    }

    pub fn clear_all() -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::delete(TOKEN_KEY);
        LocalStorage::delete(USER_DATA_KEY);
        Ok(())
    }
}
