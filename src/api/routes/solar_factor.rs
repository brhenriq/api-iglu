use actix_web::{
    // error::ReadlinesError,
    get,
    // http::{header::ContentType, StatusCode},
    // post, put,
    // web::Data,
    web::Json,
    // web::Path,
    // Responder,
    // HttpResponse,
};
// use log::info;
// use derive_more::Display;

use crate::api::modules::solar_factor::{use_case::list_solar_factor, SolarFactor};

#[get("v1/solar/factor")]
pub async fn get_task() -> Json<Vec<SolarFactor>> {
    let solar = list_solar_factor().await;

    return Json(solar);
}
