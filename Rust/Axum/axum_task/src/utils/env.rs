use std::fs;
use std::io::Write;
use std::path::Path;
use dotenvy::dotenv;

const SECRET_KEY: &str = "mysupersecuresecretkey1234567890!";

/// Function to check if `.env` file has `SECRET_KEY`, otherwise set a predefined one
pub fn init_secret_key() {
    dotenv().ok(); // Load existing .env file if present

    let env_path = ".env";
    if Path::new(env_path).exists() {
        let content = fs::read_to_string(env_path).unwrap_or_default();
        if content.contains("SECRET_KEY=") {
            return; // Secret key already exists, no need to overwrite
        }
    }

    // Write the predefined secret key
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(env_path)
        .expect("Failed to open .env file");

    writeln!(file, "SECRET_KEY={}", SECRET_KEY).expect("Failed to write to .env file");
}
