use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;

use super::error_code::ErrorCode;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,          // 是否成功
    data: Option<T>,        // 成功时的数据
    error_code: Option<String>, // 错误代码
    message: Option<String>,    // 错误信息
    timestamp: String,      // 响应时间戳
}


// Responder
impl<T: Serialize>  Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl<T> ApiResponse<T> {
    // 成功响应构造函数
    pub fn success(data: T, lang: Option<&str>) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error_code: None,
            message: None,
            timestamp: chrono::Utc::now().to_string(), // 使用当前时间戳
        }
    }
}

impl ApiResponse<()> {
    // 专用于构建错误响应的方法
    pub fn error(code: ErrorCode, lang: Option<&str>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error_code: Some(format!("{:?}", code)),
            message: Some(code.message(lang)),
            timestamp: chrono::Utc::now().to_string(),
        }
    }

}