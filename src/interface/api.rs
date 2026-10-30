use crate::domain;
use crate::interface::dto;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use domain::Storage;
use dto::ItemPayload;

pub fn get_router<S>(storage: S) -> Router
where
    S: Storage + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/items", post(set_item::<S>))
        .route("/items/{key}", get(get_item::<S>))
        .route("/items/{key}", delete(delete_item::<S>))
        .route("/healthz", get(healthcheck))
        .with_state(storage)
}

async fn set_item<S>(State(storage): State<S>, Json(payload): Json<ItemPayload>) -> StatusCode
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

async fn delete_item<S>(
    State(storage): State<S>,
    Path(key): Path<String>,
) -> Result<StatusCode, StatusCode>
where
    S: Storage,
{
    if storage.delete(&key).await.is_some() {
        Ok(StatusCode::ACCEPTED)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn healthcheck() -> StatusCode {
    StatusCode::ACCEPTED
}
