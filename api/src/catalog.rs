use std::sync::Arc;

use crate::{
    category::{Category, CATEGORY_COLLECTION},
    error::AppError,
    product::{Product, PRODUCT_COLLECTION},
    Clients, DB_NAME,
};
use axum::routing::get;
use axum::{debug_handler, Json};
use axum::{
    extract::{Query, State},
    Router,
};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Catalog {
    owner: String,
    catalog: Vec<CatalogEntry>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CatalogEntry {
    category_title: String,
    category_description: String,
    items: Vec<StreamlinedProduct>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Owner {
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StreamlinedProduct {
    title: String,
    description: String,
    price: u32,
}

impl From<Product> for StreamlinedProduct {
    fn from(product: Product) -> Self {
        Self {
            title: product.title,
            description: product.description,
            price: product.price,
        }
    }
}

#[debug_handler]
pub(crate) async fn catalog(
    State(client): State<Arc<Clients>>,
    Query(owner): Query<Owner>,
) -> Result<Json<Catalog>, AppError> {
    let db = client.mongo_client.database(DB_NAME);
    let Owner { owner } = owner;

    let product_collection = db.collection::<Product>(PRODUCT_COLLECTION);
    let category_collection = db.collection::<Category>(CATEGORY_COLLECTION);

    let filter = doc! {"owner": &owner};

    let mut category_cursor = category_collection.find(filter, None).await?;
    let mut categories = Vec::new();

    while let Some(category) = category_cursor.try_next().await? {
        categories.push(category);
    }

    let mut catalog = Vec::new();

    for category in &categories {
        let filter = doc! {"owner": &owner, "category": &category.title};

        let mut product_cursor = product_collection.find(filter, None).await?;
        let mut items = Vec::new();

        while let Some(product) = product_cursor.try_next().await? {
            items.push(product.into());
        }

        catalog.push(CatalogEntry {
            category_title: category.title.clone(),
            category_description: category.description.clone(),
            items,
        });
    }

    Ok(Json(Catalog { owner, catalog }))
}

pub(crate) fn get_catalog_routes(clients: Arc<Clients>) -> Router {
    Router::new().route("/", get(catalog)).with_state(clients)
}
