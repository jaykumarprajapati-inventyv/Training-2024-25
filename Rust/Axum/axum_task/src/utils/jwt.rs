// use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
// use serde::{Deserialize, Serialize};
// use std::env;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     sub: String,
//     exp: usize,
// }

// pub fn generate_token(user_id: &str) -> String {
//     let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
//     let claims = Claims {
//         sub: user_id.to_string(),
//         exp: 10000000000,
//     };

//     encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(secret.as_ref()),
//     )
//     .expect("Failed to generate token")
// }


/*************************************************************** */


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
    iat: usize,  // Issued At
    jti: String, // Unique Token ID
}

// Function to generate a dynamic JWT token and store it in the "User" table
pub async fn generate_token(email: &str, pool: &MySqlPool) -> String {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let iat = Utc::now().timestamp() as usize;
    let exp = (Utc::now() + Duration::hours(2)).timestamp() as usize; // Token expires in 2 hours
    let jti = Uuid::new_v4().to_string(); // Generate a unique identifier for each token

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

    // Store the token in the "token" column of the "User" table, using email as the identifier
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

