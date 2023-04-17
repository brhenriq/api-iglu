use actix_web::{get, web::Json};

use crate::api::modules::solar_factor::{use_case::list_all, SolarFactor};

#[get("v1/solar-factor")]
pub async fn list_all_solar_factor() -> Json<Vec<SolarFactor>> {
    let solar = list_all().await;

    return Json(solar);
}
