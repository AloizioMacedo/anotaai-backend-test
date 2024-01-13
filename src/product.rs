use std::sync::Arc;

use super::error::AppError;
use super::DB_NAME;

use anyhow::anyhow;
use axum::routing::{delete, patch, post};
use axum::{http::StatusCode, Router};
use serde::{Deserialize, Serialize};

use axum::extract::Query;

use mongodb::{bson::doc, Client};

use axum::extract::State;

pub(crate) const PRODUCT_COLLECTION: &str = "product";

#[derive(Serialize, Deserialize)]
pub(crate) struct Product {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) price: u32,
    pub(crate) category: String,
    pub(crate) owner: String,
}

pub(crate) async fn product(
    State(client): State<Arc<Client>>,
    Query(product): Query<Product>,
) -> Result<StatusCode, AppError> {
    let db = client.database(DB_NAME);

    let collection = db.collection::<Product>(PRODUCT_COLLECTION);

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
    State(client): State<Arc<Client>>,
    Query(association): Query<ProductAssociation>,
) -> Result<StatusCode, AppError> {
    let product_collection = client
        .database(DB_NAME)
        .collection::<Product>(PRODUCT_COLLECTION);

    let product_filter = doc! {"title": association.product};

    let category_collection = client
        .database(DB_NAME)
        .collection::<crate::category::Category>(crate::category::CATEGORY_COLLECTION);
    let category_filter = doc! {"title": &association.category};

    _ = category_collection
        .find_one(category_filter, None)
        .await?
        .ok_or(anyhow!("Category not found: {}", &association.category))?;

    product_collection
        .update_one(
            product_filter,
            doc! {"$set": {"category": association.category}},
            None,
        )
        .await?;

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProductId {
    product: String,
}

pub(crate) async fn delete_product(
    State(client): State<Arc<Client>>,
    Query(product_name): Query<ProductId>,
) -> Result<StatusCode, AppError> {
    let collection = client
        .database(DB_NAME)
        .collection::<Product>(PRODUCT_COLLECTION);

    let filter = doc! {"title": product_name.product};

    collection.delete_one(filter, None).await?;

    Ok(StatusCode::OK)
}

pub(crate) fn get_product_routes(mongodb_client: Arc<Client>) -> Router {
    Router::new()
        .route("/", post(product))
        .route("/associate", patch(associate))
        .route("/delete", delete(delete_product))
        .with_state(mongodb_client)
}
