use crate::db::establish_connection;
use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::SqlitePool;

// 引入db模块
mod db;

// type AppPool = sqlx::Pool<sqlx::Sqlite>;

#[get("/add")]
async fn add(pool: web::Data<SqlitePool>) -> impl Responder {
    let mut conn = pool.acquire().await.unwrap();

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
        "description"
    )
    .execute(&mut *conn)
    .await
    .unwrap()
    .last_insert_rowid();

    format!("id:{}", id)
}

#[get("/list")]
async fn list(pool: web::Data<SqlitePool>) -> impl Responder {
    // 从连接池中获取一个连接
    let mut conn = pool.acquire().await.unwrap();
    let recs = sqlx::query!(
        r#"
        SELECT id, description, done
        FROM todos
        ORDER BY id
                "#
    )
    .fetch_all(&mut *conn)
    .await
    .unwrap();

    let mut str = String::new(); // 创建一个空的字符串

    for rec in recs {
        // 将格式化的字符串追加到 str 中
        str.push_str(&format!(
            "- [{}] {}: {}\n",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        ));
    }

    str
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: sqlx::Pool<sqlx::Sqlite> = establish_connection().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // 共享池
            .service(
                add
            )
            .service(list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
