use async_graphql::dataloader::*;
use sqlx::PgPool;
use std::collections::HashMap;
use std::future::Future;
use uuid::Uuid;

use crate::models::Artist;

pub struct ArtistByReleaseLoader {
    pool: PgPool,
}

impl ArtistByReleaseLoader {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for ArtistByReleaseLoader {
    type Value = Vec<Artist>;
    type Error = async_graphql::Error;

    fn load(&self, keys: &[Uuid]) -> impl Future<Output = Result<HashMap<Uuid, Self::Value>, Self::Error>> + Send {
        let keys = keys.to_vec();
        let pool = self.pool.clone();
        
        async move {
            crate::dbmetrics::q();
            
            let rows = sqlx::query_as::<_, (Uuid, Uuid, String)>(
                r#"
                SELECT
                    r.gid as release_gid,
                    a.gid as artist_gid,
                    a.name as artist_name
                FROM release r
                JOIN artist_credit_name acn ON r.artist_credit = acn.artist_credit
                JOIN artist a ON acn.artist = a.id
                WHERE r.gid = ANY($1)
                ORDER BY r.gid, acn.position
                "#,
            )
            .bind(&keys)
            .fetch_all(&pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            let mut result: HashMap<Uuid, Vec<Artist>> = HashMap::new();
            
            for (release_gid, artist_gid, artist_name) in rows {
                result
                    .entry(release_gid)
                    .or_insert_with(Vec::new)
                    .push(Artist {
                        id: artist_gid.to_string(),
                        name: artist_name,
                    });
            }

            for key in &keys {
                result.entry(*key).or_insert_with(Vec::new);
            }

            Ok(result)
        }
    }
}