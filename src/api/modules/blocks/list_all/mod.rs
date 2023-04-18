use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub material_id: String,
    pub height: f64,
    pub width: f64,
    pub length: f64,
}
