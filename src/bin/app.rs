use api::route::{book::build_book_routers, health::build_health_routers};
use axum::Router;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .merge(build_health_routers())
        .merge(build_book_routers())
        .with_state(registry);
}
