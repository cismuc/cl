use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub discord_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let _ = dotenvy::dotenv();

        let discord_token =
            env::var("DISCORD_TOKEN").map_err(|_| "Missing environment variable: DISCORD_TOKEN")?;

        if discord_token.trim().is_empty() {
            return Err("DISCORD_TOKEN cannot be empty".into());
        }

        Ok(Self { discord_token })
    }
}
