use async_graphql::{ComplexObject, SimpleObject};
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
    pub id: Uuid,
    pub name: String,
}

//-----


#[derive(SimpleObject)]
// #[graphql(complex)]
pub struct Artist{
    pub id: String,
    pub name: String
}


//---------


// impl Artist{
//     async fn id(&self,ctx: Context<'_>) -> async_graphql::Result<Option<Uuid>>{
        
//     }
// }