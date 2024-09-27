use storyteller::server;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    info!("Hello, Story Teller");

    server::run().await?;
    Ok(())
}
