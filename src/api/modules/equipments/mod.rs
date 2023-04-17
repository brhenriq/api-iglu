use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Equipments {
    id: String,
    description: String,
    power: f64,
}
