use super::error::AppError;
use super::DB_NAME;
use anyhow::anyhow;
use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use axum::extract::Query;

use mongodb::Client;

use axum::extract::State;

const CATEGORY_COLLECTION: &str = "category";

#[derive(Serialize, Deserialize)]
pub(crate) struct Category {
    title: String,
    description: String,
    owner: String, // Owner ID
}

#[debug_handler]
pub(crate) async fn category(
    State(client): State<Client>,
    Query(product): Query<Category>,
) -> Result<StatusCode, AppError> {
    let db = client.database(DB_NAME);

    let collection = db.collection::<Category>(CATEGORY_COLLECTION);

    collection
        .insert_one(product, None)
        .await
        .map_err(|e| anyhow!("Error: {e}"))?;

    Ok(StatusCode::OK)
}
