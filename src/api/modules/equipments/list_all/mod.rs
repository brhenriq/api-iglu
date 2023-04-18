use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Equipments {
    pub id: String,
    pub description: String,
    pub power: f64,
}
