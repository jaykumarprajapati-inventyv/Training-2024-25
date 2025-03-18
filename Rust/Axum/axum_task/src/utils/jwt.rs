use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};
use uuid::Uuid;
use sqlx::MySqlPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize, 
    jti: String, 
}


pub async fn generate_token(email: &str, pool: &MySqlPool) -> String {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let iat = Utc::now().timestamp() as usize;
    let exp = (Utc::now() + Duration::hours(2)).timestamp() as usize; 
    let jti = Uuid::new_v4().to_string(); 

    let claims = Claims {
        sub: email.to_string(),
        exp,
        iat,
        jti: jti.clone(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Failed to generate token");

    
    sqlx::query!(
        "UPDATE User SET token = ? WHERE email = ?",
        token,
        email
    )
    .execute(pool)
    .await
    .expect("Failed to update user token in DB");

    token
}

