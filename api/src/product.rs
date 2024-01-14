use std::sync::Arc;

use super::error::AppError;
use super::Clients;
use super::DB_NAME;

use anyhow::anyhow;
use axum::routing::{delete, patch, post};
use axum::{http::StatusCode, Router};
use serde::{Deserialize, Serialize};

use axum::extract::Query;

use mongodb::bson::doc;

use axum::extract::State;

pub(crate) const PRODUCT_COLLECTION: &str = "product";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Product {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) price: u32,
    pub(crate) category: String,
    pub(crate) owner: String,
}

pub(crate) async fn product(
    State(clients): State<Arc<Clients>>,
    Query(product): Query<Product>,
) -> Result<StatusCode, AppError> {
    let db = clients.mongo_client.database(DB_NAME);

    let collection = db.collection::<Product>(PRODUCT_COLLECTION);

    collection
        .insert_one(&product, None)
        .await
        .map_err(|e| anyhow!("Error: {e}"))?;

    let arn = &clients.aws_topic_arn;
    let aws_client = &clients.aws_client;

    _ = aws_client
        .publish()
        .topic_arn(arn)
        .message(format!("owner: {}", product.owner))
        .send()
        .await
        .map_err(|e| {
            eprintln!(
                "INFO: Could not publish when creating product {:?}: {e}",
                product
            )
        });

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProductAssociation {
    product: String,
    category: String,
}

pub(crate) async fn associate(
    State(clients): State<Arc<Clients>>,
    Query(association): Query<ProductAssociation>,
) -> Result<StatusCode, AppError> {
    let client = &clients.mongo_client;

    let product_collection = client
        .database(DB_NAME)
        .collection::<Product>(PRODUCT_COLLECTION);

    let product_filter = doc! {"title": association.product};

    let category_collection = client
        .database(DB_NAME)
        .collection::<crate::category::Category>(crate::category::CATEGORY_COLLECTION);
    let category_filter = doc! {"title": &association.category};

    let cat = category_collection
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

    let arn = &clients.aws_topic_arn;
    let aws_client = &clients.aws_client;

    _ = aws_client
        .publish()
        .topic_arn(arn)
        .message(format!("owner: {}", cat.owner))
        .send()
        .await
        .map_err(|e| {
            eprintln!(
                "INFO: Could not publish when trying to associate category {:?}: {e}",
                cat
            )
        });

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProductId {
    product: String,
}

pub(crate) async fn delete_product(
    State(clients): State<Arc<Clients>>,
    Query(product_name): Query<ProductId>,
) -> Result<StatusCode, AppError> {
    let collection = clients
        .mongo_client
        .database(DB_NAME)
        .collection::<Product>(PRODUCT_COLLECTION);

    let filter = doc! {"title": &product_name.product};

    if let Some(product_in_db) = collection.find_one(filter, None).await? {
        let filter = doc! {"title": product_name.product};
        collection.delete_one(filter, None).await?;

        let arn = &clients.aws_topic_arn;
        let aws_client = &clients.aws_client;

        _ = aws_client
            .publish()
            .topic_arn(arn)
            .message(format!("owner: {}", product_in_db.owner))
            .send()
            .await
            .map_err(|e| {
                eprintln!(
                    "INFO: Could not publish when deleting category {:?}: {e}",
                    product_in_db
                )
            });

        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

pub(crate) fn get_product_routes(clients: Arc<Clients>) -> Router {
    Router::new()
        .route("/", post(product))
        .route("/associate", patch(associate))
        .route("/delete", delete(delete_product))
        .with_state(clients)
}
