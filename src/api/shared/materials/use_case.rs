use log::error;

use crate::api::modules::materials::list_all::Materials;

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

pub async fn list_material_by_id(id: &String) -> Materials {
    let query = sqlx::query_as!(
        Materials,
        "
          SELECT 
            id, 
            description, 
            conductivity  
          FROM public.materials
          WHERE id = $1;
        ",
        id.clone()
    );

    match query.fetch_one(crate::database::DATABASE.get().await).await {
        Err(_e) => {
            error!("Error on list {:?}", _e);

            Materials::default()
        }
        Ok(response) => response,
    }
}
