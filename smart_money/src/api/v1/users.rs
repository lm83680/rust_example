use actix_web::{get, put, web, HttpRequest, Responder};
use actix_web::http::header;
use crate::model::{api_response::ApiResponse, error_code::ErrorCode};

/// 初始化函数，将users内所有接口函数导出
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_user_name)
            .service(get_users_list)
            .service(put_users_list)
    );
}

/// 127.0.0.1:8080/api/v1/users/userInfo/16
#[get("/userInfo/{user_id}")]
pub async fn get_user_name(path: web::Path<String>,req: HttpRequest) -> impl Responder {
    // let user_id = path.into_inner();
    let lang = extract_language(&req).unwrap_or_else(|| "en".to_string());
    log::info!("Application started");
    log::warn!("This is a warning message");
    log::error!("This is an error message");

    ApiResponse::error(ErrorCode::InvalidEmailFormat,Some(&lang))
}

#[get("/list")]
pub async fn get_users_list()-> impl Responder {
    ApiResponse::success("user@example.com",None)
}

#[put("/userInfo")]
pub async fn put_users_list()-> impl Responder {
    ApiResponse::error(ErrorCode::InvalidEmailFormat,Some("zh-CN"))
}



/// 提取请求头中的 Accept-Language
fn extract_language(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|lang| lang.to_str().ok())
        .map(|lang| lang.to_string())
}

/// 提取请求头中的 User-Agent
fn extract_user_agent(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .map(|ua| ua.to_string())
}