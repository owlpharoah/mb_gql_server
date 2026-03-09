use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use async_graphql_axum::GraphQL;
use axum::{Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

mod models;
mod query;

use query::QueryRoot;


#[tokio::main]
async fn main(){
    let db_url = "postgres://musicbrainz:musicbrainz@localhost:5432/musicbrainz_db";
    let pool = PgPool::connect(db_url).await.expect("Failed to connect to DB");


    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).limit_depth(8).data(pool).finish();

    let app = Router::new().route_service("/gql", GraphQL::new(schema));

    println!("GraphiQL IDE: http://localhost:8000/gql");

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener,app).await.unwrap();
}
