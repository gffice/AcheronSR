use anyhow::Result;

mod game;
mod logging;
mod net;
mod util;

use logging::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    net::gateway::listen("0.0.0.0", 23301).await?;

    Ok(())
}
