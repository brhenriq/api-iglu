use actix_web::web;

use super::{
    blocks::list_all_blocks, equipments::list_all_equipments, materials::list_all_materials,
    request::request_calc, roofs::list_all_roofs, solar_factor::list_all_solar_factor,
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/v1")
        .service(list_all_solar_factor)
        .service(list_all_equipments)
        .service(list_all_materials)
        .service(list_all_blocks)
        .service(list_all_roofs)
        .service(request_calc);

    conf.service(scope);
}
