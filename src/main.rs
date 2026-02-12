use async_graphql::{Schema, EmptyMutation, EmptySubscription, dataloader::DataLoader};
use async_graphql_axum::GraphQL;
use axum::{Router, middleware::{Next}, middleware};
use axum::extract::Request; 
use axum::response::Response;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod models;
mod query;
mod dbmetrics;
mod loader;
mod cache;

use loader::ArtistByReleaseLoader;
use query::QueryRoot;
use dbmetrics::{qr, start_request};
use cache::Cache;

async fn query_counter_middleware(req: Request, next: Next) -> Response {
    start_request();  
    let response = next.run(req).await;
    qr();  
    response
}


#[tokio::main]
async fn main(){
    let db_url = "postgres://musicbrainz:musicbrainz@localhost:5432/musicbrainz_db";
    let pool = PgPool::connect(db_url).await.expect("Failed to connect to DB");
    
    let redis_url = "redis://127.0.0.1:6379";
    let cache = Cache::new(redis_url).await.expect("Failed to connect to Redis");

    let artist_loader = DataLoader::new(
        ArtistByReleaseLoader::new(pool.clone()),
        tokio::spawn,
    );

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).limit_complexity(200).limit_depth(5).data(pool).data(cache).data(artist_loader).finish();

    let app = Router::new().route_service("/gql", GraphQL::new(schema)).layer(middleware::from_fn(query_counter_middleware));

    println!("GraphiQL IDE: http://localhost:8000/gql");

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener,app).await.unwrap();
}
