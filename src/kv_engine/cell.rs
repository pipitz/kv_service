use crate::domain;
use async_trait::async_trait;
use domain::Storage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Cell {
    memory: Arc<Mutex<HashMap<String, String>>>,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            memory: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for Cell {
    async fn set(&self, key: String, value: String) {
        let mut cell = self.memory.lock().await;
        cell.insert(key, value);
    }
    async fn get(&self, key: &str) -> Option<String> {
        let cell = self.memory.lock().await;
        cell.get(key).cloned()
    }

    async fn delete(&self, key: &str) -> Option<String> {
        let mut cell = self.memory.lock().await;
        cell.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_and_get() {
        let cell = Cell::new();

        // 1. Verify a missing key returns None
        let missing = cell.get(&"missing_key".to_string()).await;
        assert_eq!(missing, None);

        // 2. Set a key-value pair and verify it can be retrieved
        cell.set("username".to_string(), "ferris".to_string()).await;
        let val = cell.get(&"username".to_string()).await;
        assert_eq!(val, Some("ferris".to_string()));

        // 3. Verify overwriting works correctly
        cell.set("username".to_string(), "rustacean".to_string())
            .await;
        let updated_val = cell.get(&"username".to_string()).await;
        assert_eq!(updated_val, Some("rustacean".to_string()));
    }

    #[tokio::test]
    async fn test_delete() {
        let cell = Cell::new();

        cell.set("username".to_string(), "ferris".to_string()).await;
        let delete_result = cell.delete(&"username".to_string()).await;
        assert_eq!(delete_result, Some("ferris".to_string()));
        let delete_result = cell.delete(&"username".to_string()).await;
        assert_eq!(delete_result, None);
    }
}
