use log::error;

use crate::api::shared::blocks::BlockWithMaterialDb;

use super::{BlockWithMaterialFormat, Material};

pub async fn list_all() -> Vec<BlockWithMaterialFormat> {
    let query = sqlx::query_as!(
        BlockWithMaterialDb,
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
        .map(|row| BlockWithMaterialFormat {
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

            [BlockWithMaterialFormat::default()].to_vec()
        }
        Ok(response) => response,
    }
}

pub async fn list_block_by_id(id: &String) -> BlockWithMaterialFormat {
    let query = sqlx::query_as!(
        BlockWithMaterialDb,
        "
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
          WHERE b.id = $1;
        ",
        id.clone()
    );

    match query
        .map(|row| BlockWithMaterialFormat {
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
        .fetch_one(crate::database::DATABASE.get().await)
        .await
    {
        Err(_e) => {
            error!("Error on list {:?}", _e);

            BlockWithMaterialFormat::default()
        }
        Ok(response) => response,
    }
}
