use log::warn;
use serde::{Deserialize, Serialize};

use crate::api::{
    modules::request::{CalcRequest, CalcResponse},
    shared::equipments::use_case::list_by_id,
    utils::calc::{
        equipment::equipment,
        insolation::insolation,
        lighting::lighting,
        peoples::peoples,
        roof::{roof, LiningProps, RoofCalcProps, TemperaturePropsRoof, TilesProps},
        wall::{
            wall, BlockProps, PlasterProps, SettlementProps, TemperaturePropsWall, WallCalcProps,
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
pub struct Request {
    peoples: Option<Vec<PeoplesRequest>>,
    equipments: Option<Vec<EquipmentsRequest>>,
}

pub async fn calc_request(p: Request) -> CalcResponse {
    let mut p_calc = 0.00;
    let mut e_calc = 0.00;

    match p.peoples {
        None => {
            warn!("Peoples not sended");
        }
        Some(p) => {
            for people in p {
                p_calc += peoples(people.activity.clone(), people.quantity.clone());
            }
        }
    }

    match p.equipments {
        None => {
            warn!("Equipments not sended");
        }
        Some(equipments) => {
            for e in equipments {
                let find_equipment = list_by_id(&e.id).await;

                e_calc += equipment(find_equipment[0].power, e.quantity);
            }
        }
    }

    let lighting = lighting(1.0);
    let insolation = insolation(1.0, 1.0);
    let wall = wall(WallCalcProps {
        block: BlockProps {
            width: 1.0,
            height: 1.0,
            length: 1.0,
            conductivity: 1.0,
        },
        plaster: PlasterProps {
            internal_thickness: 1.0,
            external_thickness: 1.0,
            conductivity: 1.0,
        },
        settlement: SettlementProps { conductivity: 1.0 },
        temperature: TemperaturePropsWall {
            internal_temperature: 1.0,
            external_temperature: 1.0,
        },
        wall_area: 1.0,
    });
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
            lighting,
            insolation,
            wall,
            roof,
            total: p_calc + e_calc + lighting + insolation + wall + roof,
        },
    }
}
