mod config;
mod database;
mod routes;
pub use database::init_db;
pub use routes::list_users_get;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Iniciando users");

    let pool = match init_db().await {
        Some(pool) => {
            tracing::info!("Initialized database");
            pool
        }
        None => {
            tracing::info!("Error when Initialized database");
            return;
        }
    };

    list_users_get(pool).await;
}
