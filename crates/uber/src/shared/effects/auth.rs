use crate::shared::models::user::User;
use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const FIREBASE_API_KEY: &str = "YOUR_FIREBASE_WEB_API_KEY"; // Placeholder
const AUTH_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthRequest {
    email: String,
    password: String,
    return_secure_token: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub local_id: String,
    pub email: String,
    pub id_token: String,
    pub refresh_token: String,
    pub expires_in: String,
}

/// Perform a Firebase REST API Login.
pub async fn login_with_email(client: &Client, email: &str, password: &str) -> Result<AuthResponse> {
    let url = format!("{}:signInWithPassword?key={}", AUTH_URL, FIREBASE_API_KEY);
    
    let req_body = AuthRequest {
        email: email.to_string(),
        password: password.to_string(),
        return_secure_token: true,
    };

    let res = client.post(&url)
        .json(&req_body)
        .send()
        .await
        .context("Network error during login")?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        anyhow::bail!("Login failed: {}", err_text);
    }

    let auth_data: AuthResponse = res.json().await.context("Failed to parse AuthResponse")?;
    Ok(auth_data)
}

/// Perform a Firebase REST API Signup.
pub async fn signup_with_email(client: &Client, email: &str, password: &str) -> Result<AuthResponse> {
    let url = format!("{}:signUp?key={}", AUTH_URL, FIREBASE_API_KEY);
    
    let req_body = AuthRequest {
        email: email.to_string(),
        password: password.to_string(),
        return_secure_token: true,
    };

    let res = client.post(&url)
        .json(&req_body)
        .send()
        .await
        .context("Network error during signup")?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        anyhow::bail!("Signup failed: {}", err_text);
    }

    let auth_data: AuthResponse = res.json().await.context("Failed to parse AuthResponse")?;
    Ok(auth_data)
}
