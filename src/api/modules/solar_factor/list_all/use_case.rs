use crate::api::shared::solar_factor::use_case::list_all;

use super::SolarFactor;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SolarFactorResponse {
    pub result: String,
    pub code: i32,
    pub data: Vec<SolarFactor>,
}

pub async fn list_all_solar_factor_request() -> SolarFactorResponse {
    let list = list_all().await;

    SolarFactorResponse {
        code: 1,
        result: "success".to_string(),
        data: list,
    }
}
