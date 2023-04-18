use log::error;

use crate::api::modules::equipments::list_all::Equipments;

pub async fn list_all() -> Vec<Equipments> {
    let query = sqlx::query_as!(
        Equipments,
        r#"
          SELECT 
            id, 
            description, 
            power  
          FROM public.equipments
          ORDER BY description ASC;
        "#
    );

    match query.fetch_all(crate::database::DATABASE.get().await).await {
        Err(_e) => {
            error!("Error on list equipments {:?}", _e);

            [Equipments::default()].to_vec()
        }
        Ok(response) => response,
    }
}
