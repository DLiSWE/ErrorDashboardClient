use anyhow::{Result, Context};
use std::env;

pub struct Config {
    pub api_port: u16,
    pub client_port: u16,
    pub environment: String,
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv::from_filename(".env.development.local").ok();

        let environment = env::var("ENVIRONMENT")
            .context("ENVIRONMENT must be set in the environment or .env file")?;

        let env_file = format!(".env.{}.local", environment);
        dotenv::from_filename(&env_file).ok();
        
        let secret_key = env::var("SECRET_KEY")
            .context("SECRET_KEY must be set in the environment or .env file")?;

        let api_port: u16 = env::var("API_PORT")
            .context("API_PORT must be set in the environment or .env file")?
            .parse()
            .context("API_PORT must be a valid number")?;

        let client_port = env::var("CLIENT_PORT")
            .context("API PORT must be set in the environemt or .env file")?
            .parse()
            .context("CLIENT_PORT must be a valid number")?;
        
        Ok(Config {
            api_port,
            client_port,
            environment,
            secret_key,
        })
    }
}
