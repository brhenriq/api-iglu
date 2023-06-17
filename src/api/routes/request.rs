use actix_web::{post, web::Json};
use serde::{Deserialize, Serialize};

use crate::api::utils::calc::{
    equipment::equipment,
    insolation::insolation,
    lighting::lighting,
    peoples::peoples,
    roof::{roof, LiningProps, RoofCalcProps, TemperaturePropsRoof, TilesProps},
    wall::{wall, BlockProps, PlasterProps, SettlementProps, TemperaturePropsWall, WallCalcProps},
};

#[derive(Deserialize, Serialize)]
pub struct Response {
    peoples: f64,
    equipments: f64,
    lighting: f64,
    insolation: f64,
    wall: f64,
    roof: f64,
    total: f64,
}

#[post("request")]
pub async fn request_calc() -> Json<Response> {
    let peoples = peoples(1.0, 1);
    let equipments = equipment(1.0, 1);
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

    Json(Response {
        peoples,
        equipments,
        lighting,
        insolation,
        wall,
        roof,
        total: peoples + equipments + lighting + insolation + wall + roof,
    })
}
