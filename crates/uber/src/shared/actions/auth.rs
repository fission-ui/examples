use crate::shared::models::user::User;

#[derive(Debug, Clone)]
pub struct LoginRequested {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct SignupRequested {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
}

#[derive(Debug, Clone)]
pub struct GoogleSignInRequested;

#[derive(Debug, Clone)]
pub struct LogoutRequested;

#[derive(Debug, Clone)]
pub struct AuthStateChanged(pub Option<User>);

#[derive(Debug, Clone)]
pub struct AuthError(pub String);

#[derive(Debug, Clone)]
pub struct AuthTokenRefreshed(pub String);
