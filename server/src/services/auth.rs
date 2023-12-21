/*
simulates a remote authentication system
 */
use std::time::Duration;

use common::hash_username;

pub enum AuthResult {
    AuthFailed,
    AuthSuccess,
}

pub async fn authuser(username: &str, nonce: u128, response: u128) -> AuthResult {
    tokio::time::sleep(Duration::from_millis(170)).await;
    if hash_username(username, nonce) == response {
        AuthResult::AuthSuccess
    } else {
        AuthResult::AuthFailed
    }
}
