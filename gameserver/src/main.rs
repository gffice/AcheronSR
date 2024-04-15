use anyhow::Result;

mod game;
mod logging;
mod net;
mod util;

use game::init_config;
use logging::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    init_config();

    net::gateway::listen("0.0.0.0", 23301).await?;

    Ok(())
}
