use actix_web::{get, web::Json};

use crate::api::modules::equipments::{use_case::list_all, Equipments};

#[get("v1/equipments")]
pub async fn list_all_equipments() -> Json<Vec<Equipments>> {
    let solar = list_all().await;

    return Json(solar);
}
