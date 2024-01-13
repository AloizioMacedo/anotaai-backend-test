use super::error::AppError;
use super::DB_NAME;
use anyhow::anyhow;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use axum::extract::Query;

use mongodb::{bson::doc, Client};

use axum::extract::State;

const PRODUCT_COLLECTION: &str = "product";

#[derive(Serialize, Deserialize)]
pub(crate) struct ProductPut {
    title: String,
    description: String,
    price: u32,
    category: String,
    owner: String, // Owner ID
}

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

#[derive(Serialize, Deserialize)]
pub(crate) struct ProductAssociation {
    product: String,
    category: String,
}

pub(crate) async fn associate(
    State(client): State<Client>,
    Query(association): Query<ProductAssociation>,
) -> Result<StatusCode, AppError> {
    let collection = client
        .database(DB_NAME)
        .collection::<ProductPut>(PRODUCT_COLLECTION);

    let filter = doc! {"owner": association.product};

    collection
        .update_one(
            filter,
            doc! {"$set": {"category": association.category}},
            None,
        )
        .await?;

    Ok(StatusCode::OK)
}
