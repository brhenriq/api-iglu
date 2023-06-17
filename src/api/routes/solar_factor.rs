use actix_web::{get, web::Json};

use crate::api::modules::solar_factor::list_all::use_case::{
    list_all_solar_factor_request, SolarFactorResponse,
};

#[get("solar-factor")]
pub async fn list_all_solar_factor() -> Json<SolarFactorResponse> {
    let solar = list_all_solar_factor_request().await;

    return Json(solar);
}
