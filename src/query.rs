use async_graphql::{Context, Object};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    Artist, ArtistRow, Recording, RecordingRow, Release, ReleaseGroup, ReleaseGroupRow, ReleaseRow,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn artist(
        &self,
        ctx: &Context<'_>,
        gid: String,
    ) -> async_graphql::Result<Option<Artist>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&gid)?;

        let row = sqlx::query_as::<_, ArtistRow>(
            "SELECT id, gid, name, sort_name
            FROM artist
            WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Artist {
            gid: r.gid,
            name: r.name,
            sort_name: r.sort_name,
            id: r.id,
        }))
    }

    async fn release_group(
        &self,
        ctx: &Context<'_>,
        gid: String,
    ) -> async_graphql::Result<Option<ReleaseGroup>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&gid)?;

        let row = sqlx::query_as::<_, ReleaseGroupRow>(
            "SELECT id, gid, name
            FROM release_group
            WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| ReleaseGroup {
            gid: r.gid,
            name: r.name,
            id: r.id,
        }))
    }

    async fn release(
        &self,
        ctx: &Context<'_>,
        gid: String,
    ) -> async_graphql::Result<Option<Release>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&gid)?;

        let row =
            sqlx::query_as::<_, ReleaseRow>(
                "SELECT id, gid, name
                FROM release
                WHERE gid = $1"
            ).bind(uuid).fetch_optional(pool).await?;

        Ok(row.map(|r| Release {
            gid: r.gid,
            name: r.name,
            id: r.id,
        }))
    }

    async fn recording(
        &self,
        ctx: &Context<'_>,
        gid: String,
    ) -> async_graphql::Result<Option<Recording>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&gid)?;

        let row = sqlx::query_as::<_, RecordingRow>(
            "SELECT gid, name, length
            FROM recording
            WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Recording {
            gid: r.gid,
            name: r.name,
            length: r.length,
        }))
    }
}
