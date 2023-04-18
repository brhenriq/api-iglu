use crate::api::shared::equipments::use_case::list_all;

use super::Equipments;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EquipmentsResponse {
    pub result: String,
    pub code: i32,
    pub data: Vec<Equipments>,
}

pub async fn list_all_equipments_request() -> EquipmentsResponse {
    let list = list_all().await;

    EquipmentsResponse {
        code: 1,
        result: "success".to_string(),
        data: list,
    }
}
