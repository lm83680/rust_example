use crate::utils::db::establish_connection;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

// 引入db模块
mod utils;
mod api;
mod model;
rust_i18n::i18n!("locales");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: sqlx::Pool<sqlx::Sqlite> = establish_connection().await.unwrap();
    rust_i18n::i18n!("app");
    
    if let Err(e) = utils::logger::setup_logging() {
        println!("日志文件初始化错误，项目未启动\n{:?}",e);
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // 可选：日志中间件
            .app_data(web::Data::new(pool.clone())) // 共享池
            .service(web::scope("/api/v1")
                .configure(api::v1::users::init)  // v1 版本的 users 路由
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
