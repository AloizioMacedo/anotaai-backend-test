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

struct Clients {
    mongo_client: Client,
    aws_client: aws_sdk_sns::Client,
    aws_topic_arn: String,
}

async fn build_mongo_client() -> Client {
    let client_options = if let Ok(url) = std::env::var("MONGO_AWS") {
        ClientOptions::parse(url).await.unwrap()
    } else {
        let mongo_service = std::env::var("MONGO_SERVICE").unwrap();
        let username = std::env::var("MONGO_USER").unwrap();
        let password = std::env::var("MONGO_PASSWORD").unwrap();

        let mut client_options = ClientOptions::parse(format!("mongodb://{mongo_service}:27017"))
            .await
            .unwrap();

        client_options.credential = Some(
            Credential::builder()
                .username(username)
                .password(password)
                .build(),
        );

        client_options
    };

    Client::with_options(client_options).unwrap()
}

async fn build_aws_sns_client() -> aws_sdk_sns::Client {
    let config = aws_config::load_from_env().await;

    aws_sdk_sns::Client::new(&config)
}

#[tokio::main]
async fn main() {
    _ = dotenv();

    eprintln!("Starting server...");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mongo_client = build_mongo_client().await;
    let aws_client = build_aws_sns_client().await;

    let aws_topic_arn = std::env::var("AWS_TOPIC_ARN").unwrap_or("".to_string());

    let clients = Arc::new(Clients {
        mongo_client,
        aws_client,
        aws_topic_arn,
    });

    let app = Router::new()
        .nest("/product", product::get_product_routes(clients.clone()))
        .nest("/category", category::get_category_routes(clients.clone()))
        .nest("/catalog", catalog::get_catalog_routes(clients.clone()))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
