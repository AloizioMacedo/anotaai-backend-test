mod catalog;
mod category;
mod error;
mod product;

use std::sync::Arc;

use axum::Router;
use dotenvy::dotenv;
use mongodb::options::{ClientOptions, Credential};
use mongodb::Client;

const DB_NAME: &str = "backend";

#[tokio::main]
async fn main() {
    _ = dotenv();

    eprintln!("Starting server...");
    let mongo_service = std::env::var("MONGO_SERVICE").unwrap();
    let username = std::env::var("MONGO_USER").unwrap();
    let password = std::env::var("MONGO_PASSWORD").unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut client_options = ClientOptions::parse(format!("mongodb://{mongo_service}:27017"))
        .await
        .unwrap();

    client_options.credential = Some(
        Credential::builder()
            .username(username)
            .password(password)
            .build(),
    );

    let client = Arc::new(Client::with_options(client_options).unwrap());

    let app = Router::new()
        .nest("/product", product::get_product_routes(client.clone()))
        .nest("/category", category::get_category_routes(client.clone()))
        .nest("/catalog", catalog::get_catalog_routes(client.clone()))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
