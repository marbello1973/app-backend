use dotenvy::dotenv;
use sqlx::{Error, Executor, Pool, Postgres, postgres::PgPoolOptions};
use std::env;
use url::Url;
 
/// Configura la base de datos: crea si no existe y devuelve el Pool
async fn config_database() -> Result<Pool<Postgres>, Error> {
    if !dotenvy::dotenv().is_ok() {
        tracing::info!("There is no configuration for the .env file")
    } else {
        dotenv().ok();
    };

    let url_database = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Parseamos la URL de forma robusta
    let db_url = Url::parse(&url_database).expect("Invalid DATABASE_URL");
    let db_name = db_url.path().trim_start_matches('/');

    // Conexión a la base de datos
    let mut base_url = db_url.clone();
    base_url.set_path("users");

    let pool_postgres = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(base_url.as_str())
        .await?;

    // Intentamos crear la DB
    let query = format!("CREATE DATABASE \"{}\";", db_name); // vamos a crear la base de datos
    // \"{}";" para proteger de nombres con espacios o caracteres especiales
    // casos posibles
    match pool_postgres.execute(query.as_str()).await {
        Ok(_) => tracing::info!("📦 Database '{}' created", db_name), // exito
        Err(sqlx::Error::Database(err)) if err.code().as_deref() == Some("42P04") => {
            tracing::info!("📦 Database  already '{}' exists", db_name)
        }
        Err(err) => return Err(err),
    }

    // Ahora sí conectamos al pool de la base real
    tracing::info!("🔗 Connected to the database '{}'", db_name);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&url_database)
        .await?;

    tracing::info!("✅ Connection established with the database '{}'", db_name);

    Ok(pool)
}

/// Ejecuta migraciones y valida si todo está correcto
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), Error> {
    tracing::info!("🚀 Running  migrations...");

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            tracing::info!("✅ Migrations applied...");
            Ok(())
        }
        Err(error) => {
            tracing::error!("❌ Error running migrations: {}", error);
            Err(error.into())
        }
    }
}

/// Inicializa la DB y aplica migraciones
pub async fn init_db() -> Option<Pool<Postgres>> {
    match config_database().await {
        Ok(pool) => {
            if let Err(e) = run_migrations(&pool).await {
                tracing::error!("❌ Migrations failed: {}", e);
                return None;
            }
            tracing::info!("Database initialized successfully ✅");
            Some(pool)
        }
        Err(e) => {
            tracing::error!("❌ Failed to initialize database: {}", e);
            None
        }
    }
}

// test unitarios
#[cfg(test)]
mod tests {
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::env;

    //funcion para cargar el archivo test
    fn load_env_test() {
        dotenvy::from_filename(".env.test").ok();
    }

    #[sqlx::test]
    async fn config_database_test() {
        load_env_test();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();

        tracing::info!("{:?}:{}", row.0, 1);
        assert_eq!(row.0, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn migrations_apply_succesfully_test(pool: PgPool) {
        let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 1);
    }
}
