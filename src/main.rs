use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;
use sqlx::{postgres::PgConnectOptions, PgPool};

struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConfig> for PgConnectOptions {
    fn from(config: DatabaseConfig) -> Self {
        PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database)
    }
    
}

fn connect_database_with(config: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(config.into())
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_db(State(pool): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&pool).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".into(),
        password: "passwd".into(),
        database: "app".into(),
    };
    let conn_pool = connect_database_with(database_config);
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(conn_pool);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

#[sqlx::test]
async fn health_check_db_works(pool: PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}
