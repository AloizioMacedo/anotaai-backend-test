mod catalog;
mod category;
mod error;
mod product;

use std::sync::Arc;

use axum::Router;
use mongodb::options::{ClientOptions, Credential};
use mongodb::Client;

const DB_NAME: &str = "backend";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    client_options.credential = Some(
        Credential::builder()
            .username("root".to_string())
            .password("example".to_string())
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
