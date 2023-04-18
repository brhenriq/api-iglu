use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SolarFactor {
    pub id: String,
    pub latitude: f64,
    pub orientation: String,
    pub value: i32,
}
