use serde::{Deserialize, Serialize};

/// Comment model matching the MongoDB collection schema.
/// Timestamps are stored as i64 (milliseconds since epoch) for cross-compilation
/// compatibility between SSR and WASM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub post_id: String,
    pub author_public_key: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    pub depth: u8,
    /// Sentiment status: 1=negative, 2=neutral, 3=positive
    pub status: u8,
    pub scoring: u8,
    pub likes_count: i32,
    pub is_deleted: bool,
    /// Milliseconds since epoch
    pub created_at: i64,
    /// Milliseconds since epoch
    pub updated_at: i64,
}

impl Comment {
    pub fn sentiment_label(&self) -> &'static str {
        match self.status {
            1 => "negative",
            3 => "positive",
            _ => "neutral",
        }
    }

    pub fn sentiment_css_class(&self) -> &'static str {
        match self.status {
            1 => "comment-negative",
            3 => "comment-positive",
            _ => "comment-neutral",
        }
    }
}
