use dotenvy::dotenv;
use sqlx::MySqlPool;
use std::env;

pub async fn connect_to_db() -> MySqlPool {
    
    dotenv().ok(); 
    
 let database_url =env::var("DATABASE_URL").expect("DB URL is required.");
   
   

    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect!!")
}



