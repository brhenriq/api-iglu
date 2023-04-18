use serde::{Deserialize, Serialize};

pub mod use_case;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Materials {
    pub id: String,
    pub description: String,
    pub conductivity: f64,
}
