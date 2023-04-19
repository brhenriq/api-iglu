use log::error;

use crate::api::shared::{roofs::RoofsFormat, roofs::RoofsWithMaterialDb};

use super::Material;

pub async fn list_all_tiles() -> Vec<RoofsFormat> {
    let query = sqlx::query_as!(
        RoofsWithMaterialDb,
        r#"
          SELECT 
            b.id,
            b.height,
            b.width,
            b.length,
            b.material_id,
            m.description,
            m.conductivity
          FROM public.blocks b
          INNER JOIN public.materials m ON m.id = b.material_id
          ORDER BY m.description ASC;
        "#
    );

    match query
        .map(|row| RoofsFormat {
            id: row.id,
            height: row.height,
            width: row.width,
            length: row.length,
            material: Material {
                id: row.material_id,
                description: row.description,
                conductivity: row.conductivity,
            },
        })
        .fetch_all(crate::database::DATABASE.get().await)
        .await
    {
        Err(_e) => {
            error!("Error on list blocks {:?}", _e);

            [RoofsFormat::default()].to_vec()
        }
        Ok(response) => response,
    }
}

pub async fn list_all_linings() -> Vec<RoofsFormat> {
    let query = sqlx::query_as!(
        RoofsWithMaterialDb,
        r#"
          SELECT 
            b.id,
            b.height,
            b.width,
            b.length,
            b.material_id,
            m.description,
            m.conductivity
          FROM public.blocks b
          INNER JOIN public.materials m ON m.id = b.material_id
          ORDER BY m.description ASC;
        "#
    );

    match query
        .map(|row| RoofsFormat {
            id: row.id,
            height: row.height,
            width: row.width,
            length: row.length,
            material: Material {
                id: row.material_id,
                description: row.description,
                conductivity: row.conductivity,
            },
        })
        .fetch_all(crate::database::DATABASE.get().await)
        .await
    {
        Err(_e) => {
            error!("Error on list blocks {:?}", _e);

            [RoofsFormat::default()].to_vec()
        }
        Ok(response) => response,
    }
}
