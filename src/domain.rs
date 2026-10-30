use async_trait::async_trait;
#[async_trait]
pub trait Storage: Send + Sync {
    async fn set(&self, key: String, value: String);
    async fn get(&self, key: &String) -> Option<String>;
}