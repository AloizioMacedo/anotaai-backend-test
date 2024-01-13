use super::error::AppError;
use super::DB_NAME;
use anyhow::anyhow;
use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use axum::extract::Query;

use mongodb::Client;

use axum::extract::State;

const PRODUCT_COLLECTION: &str = "product";

#[derive(Serialize, Deserialize)]
pub(crate) struct ProductPut {
    title: String,
    description: String,
    price: u32,
    category: String,
    owner: String,
}

#[debug_handler]
pub(crate) async fn product(
    State(client): State<Client>,
    Query(product): Query<ProductPut>,
) -> Result<StatusCode, AppError> {
    let db = client.database(DB_NAME);

    let collection = db.collection::<ProductPut>(PRODUCT_COLLECTION);

    collection
        .insert_one(product, None)
        .await
        .map_err(|e| anyhow!("Error: {e}"))?;

    Ok(StatusCode::OK)
}
