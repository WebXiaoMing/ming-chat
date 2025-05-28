mod config;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod state;
mod utils;

use axum::Router;
use tokio::net::TcpListener;

pub use config::AppConfig;
pub use state::AppState;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

pub struct App {
    pub config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn init_tracing(&self) -> &Self {
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
        self
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let config = self.config.clone();
        let addr = format!("0.0.0.0:{}", config.server.port);
        let listener = TcpListener::bind(&addr).await?;
        let state = AppState::new(config);
        let app = Router::new().nest("/api", routes::app_router(state));
        tracing::info!("Listening on: {}", addr);
        axum::serve(listener, app.into_make_service()).await?;
        Ok(())
    }
}
