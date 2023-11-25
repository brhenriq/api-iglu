use log::error;

use crate::api::modules::solar_factor::list_all::SolarFactor;

pub async fn list_all() -> Vec<SolarFactor> {
    let query = sqlx::query_as!(
        SolarFactor,
        r#"
          SELECT 
            id, 
            latitude, 
            orientation, 
            value 
          FROM public.solar_factor;
        "#
    );

    match query.fetch_all(crate::database::DATABASE.get().await).await {
        Err(_e) => {
            error!("Error on list solar_factor {:?}", _e);

            [SolarFactor::default()].to_vec()
        }
        Ok(response) => response,
    }
}

pub async fn list_solar_factor_by_id(id: &String) -> SolarFactor {
    let query = sqlx::query_as!(
        SolarFactor,
        "
          SELECT 
            id, 
            latitude, 
            orientation, 
            value 
          FROM public.solar_factor
          WHERE id = $1;
        ",
        id.clone()
    );

    match query.fetch_one(crate::database::DATABASE.get().await).await {
        Err(_e) => {
            error!("Error on list {:?}", _e);

            SolarFactor::default()
        }
        Ok(response) => response,
    }
}
