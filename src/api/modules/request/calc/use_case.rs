use log::warn;
use serde::{Deserialize, Serialize};

use crate::api::{
    modules::request::{CalcRequest, CalcResponse},
    shared::{
        blocks::use_case::list_block_by_id, equipments::use_case::list_by_id,
        materials::use_case::list_material_by_id,
    },
    utils::calc::{
        equipment::equipments_calc,
        insolation::insolation,
        lighting::lighting_calc,
        peoples::peoples_calc,
        roof::{roof, LiningProps, RoofCalcProps, TemperaturePropsRoof, TilesProps},
        wall::{
            wall_calc, BlockProps, PlasterProps, SettlementProps, TemperaturePropsWall,
            WallCalcProps,
        },
    },
};

#[derive(Deserialize, Serialize)]
pub struct PeoplesRequest {
    activity: String,
    quantity: i64,
}

#[derive(Deserialize, Serialize)]
pub struct EquipmentsRequest {
    id: String,
    quantity: i64,
}

#[derive(Deserialize, Serialize)]
pub struct WallRequest {
    block_id: String,
    plaster: PlasterRequest,
    settlement: SettlementRequest,
    area: f64,
}

#[derive(Deserialize, Serialize)]
pub struct PlasterRequest {
    material_id: String,
    internal_thickness: f64,
    external_thickness: f64,
}

#[derive(Deserialize, Serialize)]
pub struct SettlementRequest {
    material_id: String,
    conductivity: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct LightingRequest {
    area: f64,
}

#[derive(Deserialize, Serialize)]
pub struct TemperatureRequest {
    internal: f64,
    external: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    peoples: Option<Vec<PeoplesRequest>>,
    equipments: Option<Vec<EquipmentsRequest>>,
    lighting: Option<LightingRequest>,
    wall: Option<Vec<WallRequest>>,
    temperature: Option<TemperatureRequest>,
}

pub async fn calc_request(props: Request) -> CalcResponse {
    let Request {
        equipments,
        lighting,
        peoples,
        wall,
        temperature,
    } = props;

    let (internal_temperature, external_temperature) = match temperature {
        None => (0.00, 0.00),
        Some(t) => (t.internal, t.external),
    };

    let mut p_calc = 0.00;
    let mut e_calc = 0.00;
    let mut l_calc = 0.00;
    let mut w_calc = 0.00;

    match peoples {
        None => {
            warn!("Peoples not sended");
        }
        Some(p) => {
            for people in p {
                p_calc += peoples_calc(people.activity.clone(), people.quantity.clone());
            }
        }
    }

    match equipments {
        None => {
            warn!("Equipments not sended");
        }
        Some(equipments) => {
            for e in equipments {
                let find_equipment = list_by_id(&e.id).await;

                e_calc += equipments_calc(find_equipment.power, e.quantity);
            }
        }
    }

    match lighting {
        None => {
            warn!("Lighting not sended");
        }
        Some(l) => {
            l_calc += lighting_calc(l.area);
        }
    }

    let insolation = insolation(1.0, 1.0);

    match wall {
        None => {
            warn!("Wall not sended");
        }
        Some(w) => {
            for a in w {
                let block = list_block_by_id(&a.block_id).await;
                let plaster = list_material_by_id(&a.plaster.material_id).await;
                let settlement = list_material_by_id(&a.settlement.material_id).await;

                w_calc += wall_calc(WallCalcProps {
                    block: BlockProps {
                        width: block.width,
                        height: block.height,
                        length: block.length,
                        conductivity: block.material.conductivity,
                    },
                    plaster: PlasterProps {
                        internal_thickness: a.plaster.internal_thickness,
                        external_thickness: a.plaster.external_thickness,
                        conductivity: plaster.conductivity,
                    },
                    settlement: SettlementProps {
                        conductivity: match a.settlement.conductivity {
                            None => settlement.conductivity,
                            Some(conductivity) => conductivity,
                        },
                    },
                    temperature: TemperaturePropsWall {
                        internal_temperature,
                        external_temperature,
                    },
                    wall_area: a.area,
                });
            }
        }
    }

    let roof = roof(RoofCalcProps {
        temperature: TemperaturePropsRoof {
            internal_temperature: 1.0,
            external_temperature: 1.0,
        },
        lining: LiningProps {
            thickness: 1.0,
            conductivity: 1.0,
        },
        tile: TilesProps {
            thickness: 1.0,
            conductivity: 1.0,
        },
        floor_area: 1.0,
    });

    CalcResponse {
        code: 1,
        result: "success".to_string(),
        data: CalcRequest {
            peoples: p_calc,
            equipments: e_calc,
            lighting: l_calc,
            insolation,
            wall: w_calc,
            roof,
            total: p_calc + e_calc + l_calc + insolation + w_calc + roof,
        },
    }
}
