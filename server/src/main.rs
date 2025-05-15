use anyhow::Result;
use server::{App, AppConfig};

#[tokio::main]
async fn main() -> Result<()> {
    App::new(AppConfig::load()?).init_tracing().run().await
}
