use actix_web::{post, web::Json};

use crate::api::modules::request::{
    calc::use_case::{calc_request, Request},
    CalcResponse,
};

#[post("request")]
pub async fn request_calc(info: Json<Request>) -> Json<CalcResponse> {
    let response = calc_request(info.0).await;

    Json(response)
}
