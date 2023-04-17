use super::Block;
use log::error;

pub async fn list_all() -> Vec<Block> {
    let query = sqlx::query_as!(
        Block,
        r#"
          SELECT 
            id,
            material_id,
            height,
            width,
            length 
          FROM public.blocks;
        "#
    );

    match query.fetch_all(crate::database::DATABASE.get().await).await {
        Err(_e) => {
            error!("Error on list blocks {:?}", _e);

            [Block::default()].to_vec()
        }
        Ok(response) => response,
    }
}
