use serde::Deserialize;

/// Custom tag set on a resource.
#[derive(Clone, Debug, Deserialize)]
pub struct CustomTag {
    pub id: String,
    pub key: String,
    pub value: String,
}
