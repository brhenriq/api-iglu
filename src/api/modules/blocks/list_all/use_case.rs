use crate::api::shared::blocks::{use_case::list_all, BlockWithMaterialFormat};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlocksResponse {
    pub result: String,
    pub code: i32,
    pub data: Vec<BlockWithMaterialFormat>,
}

pub async fn list_all_blocks_request() -> BlocksResponse {
    let list = list_all().await;

    BlocksResponse {
        code: 1,
        result: "success".to_string(),
        data: list,
    }
}
