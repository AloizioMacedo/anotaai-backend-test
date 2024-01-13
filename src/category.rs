use std::sync::Arc;

use super::error::AppError;
use super::DB_NAME;
use anyhow::anyhow;

use axum::routing::{delete, post};
use axum::{http::StatusCode, Router};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use axum::extract::Query;

use mongodb::{bson::doc, Client};

use axum::extract::State;

pub(crate) const CATEGORY_COLLECTION: &str = "category";

#[derive(Serialize, Deserialize)]
pub(crate) struct Category {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) owner: String,
}

#[debug_handler]
pub(crate) async fn category(
    State(client): State<Arc<Client>>,
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

#[derive(Serialize, Deserialize)]
pub(crate) struct CategoryId {
    category: String,
}

pub(crate) async fn delete_category(
    State(client): State<Arc<Client>>,
    Query(category): Query<CategoryId>,
) -> Result<StatusCode, AppError> {
    let collection = client
        .database(DB_NAME)
        .collection::<Category>(CATEGORY_COLLECTION);

    let filter = doc! {"title": category.category};

    collection.delete_one(filter, None).await?;

    Ok(StatusCode::OK)
}

pub(crate) fn get_category_routes(mongodb_client: Arc<Client>) -> Router {
    Router::new()
        .route("/", post(category))
        .route("/delete", delete(delete_category))
        .with_state(mongodb_client)
}
