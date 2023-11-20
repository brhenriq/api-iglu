mod api;
mod database;

use api::routes::equipments::list_all_equipments;
use api::routes::materials::list_all_materials;
use api::routes::request::request_calc;
use api::routes::solar_factor::list_all_solar_factor;
use api::routes::{blocks::list_all_blocks, roofs::list_all_roofs};

use crate::config::config;
use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
use api::routes::{config, healthcheck::health_check_route};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub code: i64,
    pub error: bool,
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(
                actix_web::web::JsonConfig::default()
                    .limit(500000000000)
                    .error_handler(|err, _req| {
                        error::InternalError::from_response(
                            "",
                            HttpResponse::BadRequest()
                                .content_type("application/json")
                                .json(web::Json(Info {
                                    code: -3,
                                    message: err.to_string(),
                                    error: true,
                                })),
                        )
                        .into()
                    }),
            )
            .service(list_all_solar_factor)
            .service(list_all_equipments)
            .service(list_all_materials)
            .service(list_all_blocks)
            .service(list_all_roofs)
            .service(request_calc)
            .service(health_check_route)
            .configure(config)
            .wrap(logger)
    })
    .bind(("127.0.0.1", 10000))?
    .run()
    .await
}
