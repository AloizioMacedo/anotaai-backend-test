use std::sync::Arc;

use super::error::AppError;
use super::Clients;
use super::DB_NAME;
use anyhow::anyhow;

use axum::routing::{delete, post};
use axum::{http::StatusCode, Router};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use axum::extract::Query;

use mongodb::bson::doc;

use axum::extract::State;

pub(crate) const CATEGORY_COLLECTION: &str = "category";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Category {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) owner: String,
}

#[debug_handler]
pub(crate) async fn category(
    State(clients): State<Arc<Clients>>,
    Query(category): Query<Category>,
) -> Result<StatusCode, AppError> {
    let db = clients.mongo_client.database(DB_NAME);

    let collection = db.collection::<Category>(CATEGORY_COLLECTION);

    collection
        .insert_one(&category, None)
        .await
        .map_err(|e| anyhow!("Error: {e}"))?;

    let arn = &clients.aws_topic_arn;
    let aws_client = &clients.aws_client;

    _ = aws_client
        .publish()
        .topic_arn(arn)
        .message(format!("owner: {}", category.owner))
        .send()
        .await
        .map_err(|e| {
            eprintln!(
                "INFO: Could not publish when creating category {:?}: {e}",
                category
            )
        });

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CategoryId {
    category: String,
}

pub(crate) async fn delete_category(
    State(clients): State<Arc<Clients>>,
    Query(category): Query<CategoryId>,
) -> Result<StatusCode, AppError> {
    let collection = clients
        .mongo_client
        .database(DB_NAME)
        .collection::<Category>(CATEGORY_COLLECTION);

    let filter = doc! {"title": &category.category};

    if let Some(cat_in_db) = collection.find_one(filter, None).await? {
        let filter = doc! {"title": category.category};
        collection.delete_one(filter, None).await?;

        let arn = &clients.aws_topic_arn;
        let aws_client = &clients.aws_client;

        _ = aws_client
            .publish()
            .topic_arn(arn)
            .message(format!("owner: {}", cat_in_db.owner))
            .send()
            .await
            .map_err(|e| {
                eprintln!(
                    "INFO: Could not publish when deleting category {:?}: {e}",
                    cat_in_db
                )
            });

        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

pub(crate) fn get_category_routes(clients: Arc<Clients>) -> Router {
    Router::new()
        .route("/", post(category))
        .route("/delete", delete(delete_category))
        .with_state(clients)
}
