use axum::{
    extract::{Path, State},
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use dto::ItemPayload;
use domain::Storage;
use crate::domain;
use crate::interface::dto;

pub fn get_router<S>(storage: S) -> Router
where
    S: Storage + Clone + Send + Sync + 'static, {
    Router::new()
        .route("/items", post(set_item::<S>))
        .route("/items/{key}", get(get_item::<S>))
        .with_state(storage)
}

async fn set_item<S>(
    State(storage): State<S>,
    Json(payload): Json<ItemPayload>,
) -> StatusCode
where
    S: Storage,
{
    storage.set(payload.key, payload.value).await;
    StatusCode::CREATED
}

async fn get_item<S>(
    State(storage): State<S>,
    Path(key): Path<String>,
) -> Result<Json<String>, StatusCode>
where
    S: Storage,
{
    if let Some(value) = storage.get(&key).await {
        Ok(Json(value))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}