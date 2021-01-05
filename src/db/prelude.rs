use super::errors::RepositoryError;
use crate::models::users::User;

pub trait UserRepository {
    fn list(&self) -> Result<Vec<User>, RepositoryError>;
    fn add_user(&self, new_user: User) -> Result<(), RepositoryError>;
    fn get_user_by_api_key(&self, api_key: String) -> Result<User, RepositoryError>;
    fn update_user(&self, api_key: String, new_user: User) -> Result<(), RepositoryError>;
    fn delete_user(&self, api_key: String) -> Result<(), RepositoryError>;
}
