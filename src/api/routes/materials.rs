use actix_web::{get, web::Json};

use crate::api::modules::materials::{use_case::list_all, Materials};

#[get("v1/materials")]
pub async fn list_all_materials() -> Json<Vec<Materials>> {
    let materials = list_all().await;

    return Json(materials);
}
