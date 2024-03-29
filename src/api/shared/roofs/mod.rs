use serde::{Deserialize, Serialize};

pub mod use_case;

pub struct RoofsWithMaterialDb {
    pub id: String,
    pub thickness: f64,
    pub material_id: String,
    pub description: String,
    pub conductivity: f64,
    pub roof_type: String,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct RoofsFormat {
    pub id: String,
    pub roof_type: String,
    pub thickness: f64,
    pub material: Material,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Material {
    pub id: String,
    pub description: String,
    pub conductivity: f64,
}
