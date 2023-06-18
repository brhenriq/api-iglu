use actix_web::{get, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub result: String,
    pub message: String,
}

#[get("/healthcheck")]
pub async fn health_check_route() -> Json<MessageResponse> {
    return Json(MessageResponse {
        message: "server is alive".to_string(),
        result: "success".to_string(),
    });
}
