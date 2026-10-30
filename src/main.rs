mod kv_engine;
mod interface;
mod domain;

use interface::api::{
    get_router
};
use kv_engine::Cell;


#[tokio::main]
async fn main() {
    let storage: Cell = Cell::new();

    let app = get_router(storage);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await.expect("TODO: panic message");
}
