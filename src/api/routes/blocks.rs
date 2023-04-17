use actix_web::{get, web::Json};

use crate::api::modules::blocks::{use_case::list_all, Block};

#[get("v1/blocks")]
pub async fn list_all_blocks() -> Json<Vec<Block>> {
    let blocks = list_all().await;

    return Json(blocks);
}
