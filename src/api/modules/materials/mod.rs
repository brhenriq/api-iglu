use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Materials {
    id: String,
    description: String,
    conductivity: f64,
}
