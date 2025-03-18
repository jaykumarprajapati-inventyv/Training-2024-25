use std::fs;
use std::io::Write;
use std::path::Path;
use dotenvy::dotenv;

const SECRET_KEY: &str = "mysupersecuresecretkey1234567890!";


pub fn init_secret_key() {
    dotenv().ok(); 

    let env_path = ".env";
    if Path::new(env_path).exists() {
        let content = fs::read_to_string(env_path).unwrap_or_default();
        if content.contains("SECRET_KEY=") {
            return; 
        }
    }

   
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(env_path)
        .expect("Failed to open .env file");

    writeln!(file, "SECRET_KEY={}", SECRET_KEY).expect("Failed to write to .env file");
}
