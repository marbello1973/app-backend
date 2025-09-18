
use crate::handler::root_handler;

use crate::config::server_config;
use sqlx::PgPool;

async fn users_routes(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/users", axum::routing::get(root_handler))
        .with_state(pool);

    server_config(app).await.unwrap();

    Ok(())
}

// funcion para exportar la ruta
pub async fn list_users_get(pool: PgPool) {
    if let Err(error) = users_routes(pool).await {
        tracing::error!("Error al iniciar rutas: {}", error);
    }
}
