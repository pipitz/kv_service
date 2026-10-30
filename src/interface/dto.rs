use serde::Deserialize;
#[derive(Deserialize)]
pub struct ItemPayload {
    pub key: String,
    pub value: String,
}
