use crate::models::root_model;

pub fn all_user_service(){
    root_model();
    tracing::info!("User service function called");
}



/*mod models;
use models::User;

use crate::models::User;
use sqlx::{Error, PgPool};

pub struct UserService;

impl UserService {
    pub fn all_users(pool: PgPool) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(User, "SELECT id, username, email FROM users")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
}
*/