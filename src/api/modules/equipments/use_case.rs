use super::Equipments;
use log::error;

pub async fn list_all() -> Vec<Equipments> {
    let query = sqlx::query_as!(
        Equipments,
        r#"
          SELECT 
            id, 
            description, 
            power  
          FROM public.equipments;
        "#
    );

    match query.fetch_all(crate::database::DATABASE.get().await).await {
        Err(_e) => {
            error!("Error on list solar_factor {:?}", _e);

            [Equipments::default()].to_vec()
        }
        Ok(response) => response,
    }
}
