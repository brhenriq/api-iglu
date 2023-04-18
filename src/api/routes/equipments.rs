use actix_web::{get, web::Json};

use crate::api::modules::equipments::list_all::use_case::{
    list_all_equipments_request, EquipmentsResponse,
};

#[get("v1/equipments")]
pub async fn list_all_equipments() -> Json<EquipmentsResponse> {
    let equipments = list_all_equipments_request().await;

    return Json(equipments);
}
