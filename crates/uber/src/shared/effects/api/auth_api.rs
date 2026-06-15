use super::client::ApiClient;
use crate::shared::models::user::User;

pub struct AuthApi<'a> {
    pub client: &'a ApiClient,
}

impl<'a> AuthApi<'a> {
    pub async fn login(&self, _email: &str, _password: &str) -> anyhow::Result<User> {
        // Firebase Auth REST implementation would go here
        Ok(User::default())
    }
}
