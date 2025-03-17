use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// Secret key for signing JWTs
const SECRET_KEY: &str = "JayP#2240";

#[derive(Debug, Serialize, Deserialize)]

// Claims struct to store user data in the token
struct Claims {
    user_id: i32,
    exp: usize,
}

pub fn create_jwt(user_id: i32) -> String {
    let expiration_time = Utc::now() + Duration::hours(1);

    let claims = Claims {
        user_id,
        exp: expiration_time.timestamp() as usize, 
    };

    // Generate the token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .expect("Failed to create JWT");

    println!("Generated Token: {}", token); //For Debugging
    token
}

//To validate the JWT
pub fn validate_jwt(token: &str) -> Result<i32, String> {
    // Remove "Bearer " prefix if present
    let token = token
        .trim_start_matches("Bearer ")
        .trim_matches(|c| c == '<' || c == '>');

    println!("Token to Validate: {}", token);  //For Debugging

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &Validation::default(),
    );

    println!("Token'ss data: {:?}", token_data); //For Debugging

    match token_data {
        Ok(data) => {
            println!("Valid Token: User ID = {}", data.claims.user_id); //For Debugging
            Ok(data.claims.user_id)
        }
        Err(e) => {
            println!("Token Validation Error: {:?}", e); //For Debugging
            Err("Invalid token".to_string())
        }
    }
}
