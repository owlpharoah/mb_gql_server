use async_graphql::{Context, Object};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dbmetrics, models::{Artist,ArtistRow, Release,ReleaseRow}, cache::Cache};

pub struct QueryRoot;


#[Object]
impl QueryRoot{
    async fn artist(&self,ctx:&Context<'_>,id: String) -> async_graphql::Result<Option<Artist>>{
        let cache = ctx.data::<Cache>()?;
        let mut cache = cache.clone();
        
        //try cache
        let cache_key = Cache::artist_key(&id);
        if let Some(cached) = cache.get::<Artist>(&cache_key).await {
            eprintln!("  💾 Cache HIT for artist {}", id);
            return Ok(Some(cached));
        }
        
        eprintln!("  ❌ Cache MISS for artist {}", id);
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

        if let Some(r) = row {
            let artist = Artist {
                id: r.gid.to_string(),
                name: r.name,
            };
            
        // cache for 24 hours
            cache.set(&cache_key, &artist, 86400).await;
            eprintln!("  ✅ Cached artist {}", id);
            
            Ok(Some(artist))
        } else {
            Ok(None)
        }
    }

    async fn artists(&self,ctx:&Context<'_>,ids: Vec<String>) -> async_graphql::Result<Vec<Artist>>{
        if ids.len() > 1000 {
            return Err("Too many IDs".into());
        }
        dbmetrics::q();
        let pool = ctx.data::<PgPool>()?;
        let uuids: Result<Vec<Uuid>, uuid::Error> = ids.into_iter().map(|r| Uuid::parse_str(&r)).collect();

        let row: Vec<ArtistRow> = sqlx::query_as::<_,ArtistRow>(
            r#"
            SELECT 
                a.gid, 
                a.name
            FROM artist a
            WHERE a.gid = ANY($1)
            "#
        ).bind(uuids?).fetch_all(pool).await?;

        Ok(row.into_iter().map(|r| Artist{
            id: r.gid.to_string(),
            name: r.name
        }).collect())
    }

    async fn release(&self,ctx:&Context<'_>,id:String) -> async_graphql::Result<Option<Release>>{
        let cache = ctx.data::<Cache>()?;
        let mut cache = cache.clone();
        
        
        let cache_key = Cache::release_key(&id);
        if let Some(cached) = cache.get::<Release>(&cache_key).await {
            eprintln!("  💾 Cache HIT for release {}", id);
            return Ok(Some(cached));
        }
        
        eprintln!("  ❌ Cache MISS for release {}", id);
        dbmetrics::q();
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&id)?;

        let row = sqlx::query_as::<_,ReleaseRow>(
            r#"
            SELECT 
                r.gid,
                r.name
                FROM release r
                WHERE r.gid=$1
            "#
        ).bind(uuid).fetch_optional(pool).await?;

        if let Some(r) = row {
            let release = Release {
                gid: r.gid.to_string(),
                name: r.name,
            };
            
            // cache for 12 hours
            cache.set(&cache_key, &release, 43200).await;
            eprintln!("  ✅ Cached release {}", id);
            
            Ok(Some(release))
        } else {
            Ok(None)
        }
    }

    async fn search_artist(&self, ctx:&Context<'_>,name:String, limit: Option<i32>, offset: Option<i32>) -> async_graphql::Result<Vec<Artist>>{
        dbmetrics::q();
        let pool = ctx.data::<PgPool>()?;
        
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);

        let pattern = format!("%{}%", name);

        let rows = sqlx::query_as::<_,ArtistRow>(
            r#"
            SELECT
                a.gid,
                a.name
            FROM artist a
            WHERE a.name ILIKE $1
            LIMIT $2 OFFSET $3
            "#
        ).bind(&pattern).bind(&limit).bind(&offset).fetch_all(pool).await?;

        Ok(rows.into_iter().map(|r| Artist{
            id: r.gid.to_string(),
            name:r.name
        }).collect())
    }
}