use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use async_graphql_axum::GraphQL;
use axum::{Router, middleware::{Next}, middleware};
use axum::extract::Request;  // ← Use axum's Request
use axum::response::Response; // ← Use axum's Response 
use sqlx::PgPool;
use tokio::net::TcpListener;

mod models;
mod query;
mod dbmetrics;

use query::QueryRoot;
use dbmetrics::qr;

async fn query_counter_middleware(req: Request, next: Next) -> Response {
    let response = next.run(req).await;
    qr(); // Reset and print total
    response
}


#[tokio::main]
async fn main(){
    let db_url = "postgres://musicbrainz:musicbrainz@localhost:5432/musicbrainz_db";
    let pool = PgPool::connect(db_url).await.expect("Failed to connect to DB");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(pool).finish();

    let app = Router::new().route_service("/gql", GraphQL::new(schema)).layer(middleware::from_fn(query_counter_middleware));

    println!("GraphiQL IDE: http://localhost:8000/gql");

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener,app).await.unwrap();
}