use actix_web::{
    get,
    web::{Json, Query},
};
use serde::Deserialize;

use crate::api::modules::roofs::list_all::use_case::{list_all_roofs_request, RoofsResponse};

#[derive(Deserialize, Debug)]
pub struct Info {
    roof_type: Option<i32>,
}

#[get("roofs")]
pub async fn list_all_roofs(info: Query<Info>) -> Json<RoofsResponse> {
    let equipments = list_all_roofs_request(info.roof_type).await;

    return Json(equipments);
}
