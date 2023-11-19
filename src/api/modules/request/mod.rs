pub mod calc;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CalcRequest {
    peoples: f64,
    equipments: f64,
    lighting: f64,
    insolation: f64,
    wall: f64,
    roof: f64,
    total: f64,
}

#[derive(Deserialize, Serialize)]
pub struct CalcResponse {
    pub result: String,
    pub code: i32,
    pub data: CalcRequest,
}
