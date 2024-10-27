use crate::{state::AppState, store::Store, views::create_views_router};

mod error;
mod state;
mod store;
mod views;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(err) = dotenvy::dotenv() {
        log::warn!("Failed to init dotenvy: {err}");
    }

    let database_url = std::env::var("DATABASE_URL")
        .expect("Expected DATABASE_URL environment variable to be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    let state = AppState::new(Store::new(pool));

    let router = axum::Router::new()
        .merge(create_views_router())
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(state);

    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()?;
    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).await?;
    log::info!("Server running on http://{}", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}
