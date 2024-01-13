mod category;
mod error;
mod product;

use axum::routing::{get, post, put};
use axum::Router;
use mongodb::options::{ClientOptions, Credential};
use mongodb::Client;

const DB_NAME: &str = "backend";

#[tokio::main]
async fn main() {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    client_options.credential = Some(
        Credential::builder()
            .username("root".to_string())
            .password("example".to_string())
            .build(),
    );

    let client = Client::with_options(client_options).unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/product", post(product::product))
        .route("/product/associate", put(product::associate))
        .route("/category", post(category::category))
        .with_state(client);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
