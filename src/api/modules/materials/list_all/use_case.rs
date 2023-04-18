use crate::api::shared::materials::use_case::list_all;

use super::Materials;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MaterialsResponse {
    pub result: String,
    pub code: i32,
    pub data: Vec<Materials>,
}

pub async fn list_all_materials_request() -> MaterialsResponse {
    let list = list_all().await;

    MaterialsResponse {
        code: 1,
        result: "success".to_string(),
        data: list,
    }
}
