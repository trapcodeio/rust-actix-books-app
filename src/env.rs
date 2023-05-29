pub struct Env {
    pub env: String,
    pub app_name: String,
    pub app_port: u16,
    pub mongodb_server: String,
    pub mongodb_database: String,
}

impl Env {
    pub fn is_development(&self) -> bool {
        self.env == "development"
    }
}


impl Clone for Env {
    fn clone(&self) -> Self {
        Env {
            env: self.env.to_string(),
            app_name: self.app_name.to_string(),
            app_port: self.app_port,
            mongodb_server: self.mongodb_server.to_string(),
            mongodb_database: self.mongodb_database.to_string(),
        }
    }
}

pub fn load_env() -> Env {
    dotenv::dotenv().ok();

    Env {
        env: std::env::var("ENV").unwrap_or("development".to_string()),
        app_name: std::env::var("APP_NAME").unwrap_or("Books App".to_string()),
        app_port: std::env::var("APP_PORT").unwrap_or("9000".to_string()).parse::<u16>().unwrap(),
        mongodb_server: std::env::var("MONGODB_SERVER").unwrap_or("mongodb://127.0.0.1:27017".to_string()),
        mongodb_database: std::env::var("MONGODB_DATABASE").unwrap_or("books".to_string()),
    }
}
