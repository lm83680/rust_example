use sqlx::sqlite::{Sqlite, SqlitePool};
use sqlx::Pool;

pub async fn establish_connection() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = "sqlite://smart_money.db".to_string();
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}
