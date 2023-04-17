use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    id: String,
    material_id: String,
    height: f64,
    width: f64,
    length: f64,
}
