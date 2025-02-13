use sqlx::sqlite::{Sqlite, SqlitePool};
use sqlx::Pool;
use std::env;

pub async fn establish_connection() -> Result<Pool<Sqlite>, sqlx::Error> {
    // 使用环境变量来配置数据库路径，确保生产环境和开发环境能够适配
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://local.db".to_string());

    let pool = SqlitePool::connect(&database_url).await?;

    Ok(pool)
}
