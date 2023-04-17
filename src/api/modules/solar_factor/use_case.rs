use super::SolarFactor;
use log::error;

pub async fn list_solar_factor() -> Vec<SolarFactor> {
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
