use log::error;

use crate::api::shared::{roofs::RoofsFormat, roofs::RoofsWithMaterialDb};

use super::Material;

pub async fn list_all(roof_type: Option<i32>) -> Vec<RoofsFormat> {
    let query = sqlx::query_as!(
        RoofsWithMaterialDb,
        r#"
          SELECT 
            r.id,
            r.thickness,
            r.material_id,
            rt.description as roof_type,
            m.description,
            m.conductivity
          FROM public.roofs r
          INNER JOIN public.materials m ON m.id = r.material_id
          INNER JOIN public.roofs_types rt ON rt.id = r.type_id
          WHERE r.type_id = $1
          ORDER BY m.description ASC;
        "#,
        roof_type
    );

    match query
        .map(|row| RoofsFormat {
            id: row.id,
            thickness: row.thickness,
            roof_type: row.roof_type,
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
            error!("Error on list roofs {:?}", _e);

            [RoofsFormat::default()].to_vec()
        }
        Ok(response) => response,
    }
}
