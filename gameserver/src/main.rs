use anyhow::Result;

mod game;
mod logging;
mod net;
mod util;

use logging::init_tracing;

const DEFAULT_DOTENV: &str = include_str!("../.env");

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    init_config()?;

    net::gateway::listen("0.0.0.0", 23301).await?;
    Ok(())
}

fn init_config() -> Result<()> {
    let local_dotenv = std::path::Path::new(".env");
    if local_dotenv.exists() {
        dotenv::dotenv()?;
    } else {
        let config = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("No config directory found"))?
            .join("hkrpg-gameserver");

        std::fs::create_dir_all(&config)?;

        let env = config.join(".env");

        if !env.exists() {
            std::fs::write(&env, DEFAULT_DOTENV)?;
        }

        dotenv::from_path(&env)?;
    }

    Ok(())
}
