use anyhow::Result;

mod game;
mod net;
mod util;

use common::data::init_assets;
use common::logging::init_tracing;
use game::init_config;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    init_config();
    init_assets();

    net::gateway::listen("0.0.0.0", 23301).await?;

    Ok(())
}
