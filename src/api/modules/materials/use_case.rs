use super::Materials;
use log::error;

pub async fn list_all() -> Vec<Materials> {
    let query = sqlx::query_as!(
        Materials,
        r#"
          SELECT 
            id, 
            description, 
            conductivity  
          FROM public.materials;
        "#
    );

    match query.fetch_all(crate::database::DATABASE.get().await).await {
        Err(_e) => {
            error!("Error on list materials {:?}", _e);

            [Materials::default()].to_vec()
        }
        Ok(response) => response,
    }
}
