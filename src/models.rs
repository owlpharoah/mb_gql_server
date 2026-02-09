use async_graphql::{ComplexObject, SimpleObject, Context};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow)]
pub struct ArtistRow{
    pub gid: Uuid,
    pub name: String,
}

#[derive(FromRow)]
pub struct ArtistOptional{
    pub type_name: Option<String>,
    pub gender: Option<i32>
}

#[derive(FromRow)]
pub struct ReleaseRow{
    pub gid: Uuid,
    pub name: String,
}

//-----


#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Artist{
    pub id: String,
    pub name: String
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct  Release{
    pub gid: String,
    pub name: String
}

//---------

#[ComplexObject]
impl Artist{
    async fn release(&self,ctx: &Context<'_>, limit: Option<i32>, offset:Option<i32>) -> async_graphql::Result<Vec<Release>>{
        let pool = ctx.data::<PgPool>()?;
        
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query_as::<_,ReleaseRow>(
            r#"
                SELECT
                r.gid,
                r.name
            FROM release r
            JOIN artist_credit_name acn ON r.artist_credit = acn.artist_credit
            JOIN artist a ON acn.artist = a.id
            WHERE a.gid = $1
            LIMIT $2 OFFSET $3
            "#
        ).bind(Uuid::parse_str(&self.id)?).bind(&limit).bind(&offset).fetch_all(pool).await?;

        Ok(rows.into_iter().map(|r| Release{
            gid:r.gid.to_string(),
            name:r.name
        }).collect())
    }
}

#[ComplexObject]
impl Release{
    async fn artist(&self,ctx:&Context<'_>) -> async_graphql::Result<Vec<Artist>>{
        let pool = ctx.data::<PgPool>()?;
        let rows = sqlx::query_as::<_,ArtistRow>(
            r#"
            SELECT
                a.gid,
                a.name
            FROM release r
            JOIN artist_credit_name acn
                ON r.artist_credit = acn.artist_credit
            JOIN artist a
                ON acn.artist = a.id
            WHERE r.gid = $1;
            "#
        ).bind(Uuid::parse_str(&self.gid)?).fetch_all(pool).await?;

        Ok(rows.into_iter().map(|r| Artist{
            id: r.gid.to_string(),
            name:r.name
        }).collect())
    }
}