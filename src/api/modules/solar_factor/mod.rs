use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SolarFactor {
    id: String,
    latitude: f64,
    orientation: String,
    value: i32,
}
