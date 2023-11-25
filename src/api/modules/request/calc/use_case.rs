use log::warn;
use serde::{Deserialize, Serialize};

use crate::api::{
    modules::request::{CalcRequest, CalcResponse},
    shared::{
        blocks::use_case::list_block_by_id, equipments::use_case::list_by_id,
        materials::use_case::list_material_by_id, roofs::use_case::list_roof_by_id,
        solar_factor::use_case::list_solar_factor_by_id,
    },
    utils::calc::{
        equipment::equipments_calc,
        insolation::insolation_calc,
        lighting::lighting_calc,
        peoples::peoples_calc,
        roof::{roof_calc, LiningProps, RoofCalcProps, TemperaturePropsRoof, TilesProps},
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
pub struct RoofRequest {
    lining_id: String,
    tile_id: String,
    area: f64,
}

#[derive(Deserialize, Serialize)]
pub struct InsolationRequest {
    solar_factor_id: String,
    area: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    peoples: Option<Vec<PeoplesRequest>>,
    equipments: Option<Vec<EquipmentsRequest>>,
    lighting: Option<LightingRequest>,
    wall: Option<Vec<WallRequest>>,
    roof: Option<RoofRequest>,
    temperature: Option<TemperatureRequest>,
    glass: Option<InsolationRequest>,
}

pub async fn calc_request(props: Request) -> CalcResponse {
    let Request {
        equipments,
        lighting,
        peoples,
        wall,
        temperature,
        roof,
        glass,
    } = props;

    let (internal_temperature, external_temperature) = match temperature {
        None => (0.00, 0.00),
        Some(_temperature) => (_temperature.internal, _temperature.external),
    };

    let mut peoples_result = 0.00;
    let mut equipments_result = 0.00;
    let mut lighting_result = 0.00;
    let mut walls_result = 0.00;
    let mut roof_result = 0.00;
    let mut insolation_result = 0.00;

    match peoples {
        None => {
            warn!("Peoples not sended");
        }
        Some(_peoples) => {
            for people in _peoples {
                peoples_result += peoples_calc(people.activity.clone(), people.quantity.clone());
            }
        }
    }

    match equipments {
        None => {
            warn!("Equipments not sended");
        }
        Some(_equipments) => {
            for equipment in _equipments {
                let find_equipment = list_by_id(&equipment.id).await;

                equipments_result += equipments_calc(find_equipment.power, equipment.quantity);
            }
        }
    }

    match lighting {
        None => {
            warn!("Lighting not sended");
        }
        Some(_lighting) => {
            lighting_result += lighting_calc(_lighting.area);
        }
    }

    match glass {
        None => {
            warn!("Insolation not sended");
        }
        Some(_glass) => {
            let solar_factor = list_solar_factor_by_id(&_glass.solar_factor_id).await;

            insolation_result += insolation_calc(solar_factor.value as f64, _glass.area);
        }
    }

    match wall {
        None => {
            warn!("Wall not sended");
        }
        Some(_wall) => {
            for wall in _wall {
                let block = list_block_by_id(&wall.block_id).await;
                let plaster = list_material_by_id(&wall.plaster.material_id).await;
                let settlement = list_material_by_id(&wall.settlement.material_id).await;

                walls_result += wall_calc(WallCalcProps {
                    block: BlockProps {
                        width: block.width,
                        height: block.height,
                        length: block.length,
                        conductivity: block.material.conductivity,
                    },
                    plaster: PlasterProps {
                        internal_thickness: wall.plaster.internal_thickness,
                        external_thickness: wall.plaster.external_thickness,
                        conductivity: plaster.conductivity,
                    },
                    settlement: SettlementProps {
                        conductivity: match wall.settlement.conductivity {
                            None => settlement.conductivity,
                            Some(conductivity) => conductivity,
                        },
                    },
                    temperature: TemperaturePropsWall {
                        internal_temperature,
                        external_temperature,
                    },
                    wall_area: wall.area,
                });
            }
        }
    }

    match roof {
        None => {
            warn!("Lighting not sended");
        }
        Some(_roof) => {
            let lining = list_roof_by_id(&_roof.lining_id).await;
            let tile = list_roof_by_id(&_roof.tile_id).await;

            roof_result += roof_calc(RoofCalcProps {
                temperature: TemperaturePropsRoof {
                    internal_temperature,
                    external_temperature,
                },
                lining: LiningProps {
                    thickness: lining.thickness,
                    conductivity: lining.material.conductivity,
                },
                tile: TilesProps {
                    thickness: tile.thickness,
                    conductivity: tile.material.conductivity,
                },
                floor_area: _roof.area,
            });
        }
    }

    CalcResponse {
        code: 1,
        result: "success".to_string(),
        data: CalcRequest {
            peoples: peoples_result,
            equipments: equipments_result,
            lighting: lighting_result,
            insolation: insolation_result,
            wall: walls_result,
            roof: roof_result,
            total: peoples_result
                + equipments_result
                + lighting_result
                + insolation_result
                + walls_result
                + roof_result,
        },
    }
}
