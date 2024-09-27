use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

use crate::{service, AppConfig, AppState};

pub async fn run() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    dotenv::dotenv().ok();

    let value = dotenv::var("DATABASE_URL").expect("expect database URL");
    let config = AppConfig {
        port: 8888,
        db_url: value,
    };

    let state = AppState::new(&config).await?;

    let addr = format!("0.0.0.0:{}", &config.port);

    let app = service::set_app_router(state)?;
    let listener = TcpListener::bind(&addr).await?;
    info!("listening on: {}", config.port);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
