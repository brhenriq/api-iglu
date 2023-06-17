use actix_web::{get, web::Json};

use crate::api::modules::materials::list_all::use_case::{
    list_all_materials_request, MaterialsResponse,
};

#[get("materials")]
pub async fn list_all_materials() -> Json<MaterialsResponse> {
    let materials = list_all_materials_request().await;

    return Json(materials);
}
