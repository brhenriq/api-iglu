use crate::api::shared::roofs::{use_case::list_all, RoofsFormat};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoofsResponse {
    pub result: String,
    pub code: i32,
    pub data: Vec<RoofsFormat>,
}

pub async fn list_all_roofs_request(roof_type: Option<i32>) -> RoofsResponse {
    let list = list_all(roof_type).await;

    RoofsResponse {
        code: 1,
        result: "success".to_string(),
        data: list,
    }
}
