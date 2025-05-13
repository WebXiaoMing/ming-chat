use anyhow::{Ok, Result};
use chat::{AppConfig, get_routes};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;
    let fmt_layer = fmt::layer()
        .with_target(false) // don't include event targets when logging
        .with_level(false)
        .compact();

    // 2. 文件日志（每日轮换）
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "app.log");

    let file_layer = fmt::layer().with_writer(file_appender).json(); // JSON 格式输出

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .with(fmt_layer)
        .with(file_layer)
        .init();
    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    let app = get_routes(config);
    axum::serve(listener, app).await?;
    Ok(())
}
