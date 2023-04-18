use actix_web::{get, web::Json};

use crate::api::modules::blocks::list_all::use_case::{list_all_blocks_request, BlocksResponse};

#[get("v1/blocks")]
pub async fn list_all_blocks() -> Json<BlocksResponse> {
    let blocks = list_all_blocks_request().await;

    return Json(blocks);
}
