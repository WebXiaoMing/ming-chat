mod config;
mod handlers;
mod routes;
mod state;

use axum::Router;
pub use config::AppConfig;
pub use state::AppState;

pub fn get_routes(config: AppConfig) -> Router {
    let state = AppState::new(config);

    Router::new().nest("/api", routes::api(state))
}
