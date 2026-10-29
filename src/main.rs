mod kv_engine;

use kv_engine::Cell;
use crate::kv_engine::Storage;

fn main() {
    let mut cell = Cell::new();
    cell.set(String::from("hello"), 1);
    cell.get(String::from("hello"));
}
