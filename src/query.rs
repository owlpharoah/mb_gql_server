use async_graphql::{Context, Object};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dbmetrics, models::{Artist,ArtistRow}};

pub struct QueryRoot;


#[Object]
impl QueryRoot{
    async fn artist(&self,ctx:&Context<'_>,id: String) -> async_graphql::Result<Option<Artist>>{
        dbmetrics::q();
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&id)?;

        let row: Option<ArtistRow> = sqlx::query_as::<_,ArtistRow>(
            r#"
            SELECT 
                a.gid, 
                a.name
            FROM artist a
            WHERE a.gid = $1
            "#
        ).bind(uuid).fetch_optional(pool).await?;

        Ok(row.map(|r| Artist{
            id: r.gid.to_string(),
            name: r.name
        }))
    }
}