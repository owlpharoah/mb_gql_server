use async_graphql::{ComplexObject, SimpleObject, Context};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(FromRow)]
pub struct ArtistRow {
    pub id: i32,
    pub gid: Uuid,
    pub name: String,
    pub sort_name: String,
}

#[derive(FromRow)]
pub struct ReleaseGroupRow {
    pub id: i32,
    pub gid: Uuid,
    pub name: String,
}

#[derive(FromRow)]
pub struct ReleaseRow {
    pub id: i32,
    pub gid: Uuid,
    pub name: String,
}

#[derive(FromRow)]
pub struct RecordingRow {
    pub gid: Uuid,
    pub name: String,
    pub length: Option<i32>,
}

/--------------------

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Artist {
    pub gid: Uuid,
    pub name: String,
    pub sort_name: String,
    #[graphql(skip)]
    pub id: i32,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct ReleaseGroup {
    pub gid: Uuid,
    pub name: String,
    #[graphql(skip)]
    pub id: i32,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Release {
    pub gid: Uuid,
    pub name: String,
    #[graphql(skip)]
    pub id: i32,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Recording {
    pub gid: Uuid,
    pub name: String,
    pub length: Option<i32>,
}

#[ComplexObject]
impl Artist {
    async fn artist_type(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<String>> {
        let pool = ctx.data::<PgPool>()?;
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT at.name
            FROM artist_type at JOIN artist a ON a.type = at.id
            WHERE a.id = $1",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| r.0))
    }

    async fn gender(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<String>> {
        let pool = ctx.data::<PgPool>()?;
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT g.name
            FROM gender g JOIN artist a ON a.gender = g.id
            WHERE a.id = $1",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| r.0))
    }

    async fn area(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<String>> {
        let pool = ctx.data::<PgPool>()?;
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT ar.name
            FROM area ar JOIN artist a ON a.area = ar.id
            WHERE a.id = $1",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| r.0))
    }

    async fn release_groups(
        &self,
        ctx: &Context<'_>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> async_graphql::Result<Vec<ReleaseGroup>> {
        let pool = ctx.data::<PgPool>()?;
        let rows = sqlx::query_as::<_, ReleaseGroupRow>(
            r#"
            SELECT rg.id, rg.gid, rg.name
            FROM release_group rg
            JOIN artist_credit_name acn ON rg.artist_credit = acn.artist_credit
            WHERE acn.artist = $1
            ORDER BY rg.name
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(self.id)
        .bind(limit.unwrap_or(5))
        .bind(offset.unwrap_or(0))
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| ReleaseGroup {
                gid: r.gid,
                name: r.name,
                id: r.id,
            })
            .collect())
    }
}

#[ComplexObject]
impl ReleaseGroup {
    async fn releases(
        &self,
        ctx: &Context<'_>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> async_graphql::Result<Vec<Release>> {
        let pool = ctx.data::<PgPool>()?;
        let rows = sqlx::query_as::<_, ReleaseRow>(
            r#"
            SELECT r.id, r.gid, r.name
            FROM release r
            WHERE r.release_group = $1
            ORDER BY r.name
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(self.id)
        .bind(limit.unwrap_or(25))
        .bind(offset.unwrap_or(0))
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Release {
                gid: r.gid,
                name: r.name,
                id: r.id,
            })
            .collect())
    }
}

#[ComplexObject]
impl Release {
    async fn recordings(
        &self,
        ctx: &Context<'_>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> async_graphql::Result<Vec<Recording>> {
        let pool = ctx.data::<PgPool>()?;
        let rows = sqlx::query_as::<_, RecordingRow>(
            r#"
            SELECT DISTINCT rec.gid, rec.name,rec.length
            FROM recording rec
            JOIN track t ON t.recording = rec.id
            JOIN medium m ON t.medium = m.id
            WHERE m.release = $1
            ORDER BY rec.name
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(self.id)
        .bind(limit.unwrap_or(25))
        .bind(offset.unwrap_or(0))
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Recording {
                gid: r.gid,
                name: r.name,
                length: r.length,
            })
            .collect())
    }
}
