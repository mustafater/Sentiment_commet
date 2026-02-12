use bson::{doc, oid::ObjectId, Document};
use futures::TryStreamExt;
use crate::model::Comment;
use super::db::get_comments_collection;
use super::sentiment::{analyze_sentiment, compute_scoring};

/// Convert a BSON Document to our Comment model.
fn doc_to_comment(doc: &Document) -> Option<Comment> {
    Some(Comment {
        id: doc.get_object_id("_id").ok().map(|id| id.to_hex()),
        post_id: doc.get_str("post_id").ok()?.to_string(),
        author_public_key: doc.get_str("author_public_key").ok()?.to_string(),
        content: doc.get_str("content").ok()?.to_string(),
        parent_id: doc.get_object_id("parent_id").ok().map(|id| id.to_hex()),
        depth: doc.get_i32("depth").ok().unwrap_or(0) as u8,
        status: doc.get_i32("status").ok().unwrap_or(2) as u8,
        scoring: doc.get_i32("scoring").ok().unwrap_or(0) as u8,
        likes_count: doc.get_i32("likes_count").ok().unwrap_or(0),
        is_deleted: doc.get_bool("is_deleted").ok().unwrap_or(false),
        created_at: doc.get_datetime("created_at")
            .ok()
            .map(|dt| dt.timestamp_millis())
            .unwrap_or(0),
        updated_at: doc.get_datetime("updated_at")
            .ok()
            .map(|dt| dt.timestamp_millis())
            .unwrap_or(0),
    })
}

/// Create a new comment.
pub async fn create_comment(
    post_id: &str,
    author_public_key: &str,
    content: &str,
    parent_id: Option<String>,
    depth: u8,
) -> Result<Comment, String> {
    let col = get_comments_collection();
    let now = bson::DateTime::now();

    let sentiment_status = analyze_sentiment(content);
    let scoring = compute_scoring(content);

    // Store post_id as a string (supports both ObjectId and human-readable IDs)

    let parent_oid = match &parent_id {
        Some(pid) => Some(ObjectId::parse_str(pid).map_err(|e| format!("Invalid parent_id: {}", e))?),
        None => None,
    };

    let mut doc = doc! {
        "post_id": post_id,
        "author_public_key": author_public_key,
        "content": content,
        "depth": depth as i32,
        "status": sentiment_status as i32,
        "scoring": scoring as i32,
        "likes_count": 0_i32,
        "is_deleted": false,
        "created_at": now,
        "updated_at": now,
    };

    if let Some(pid) = parent_oid {
        doc.insert("parent_id", pid);
    }

    let result = col.insert_one(doc).await.map_err(|e| format!("Insert error: {}", e))?;

    let inserted_id = result
        .inserted_id
        .as_object_id()
        .ok_or("No inserted ID returned")?;

    let id_str = inserted_id.to_hex();

    Ok(Comment {
        id: Some(id_str),
        post_id: post_id.to_string(),
        author_public_key: author_public_key.to_string(),
        content: content.to_string(),
        parent_id,
        depth,
        status: sentiment_status,
        scoring,
        likes_count: 0,
        is_deleted: false,
        created_at: now.timestamp_millis(),
        updated_at: now.timestamp_millis(),
    })
}

/// Get all non-deleted comments for a post, sorted by creation time.
pub async fn get_comments_by_post(post_id: &str) -> Result<Vec<Comment>, String> {
    let col = get_comments_collection();
    // Use string-based post_id matching

    let filter = doc! {
        "post_id": post_id,
        "is_deleted": false,
    };

    let mut cursor = col
        .find(filter)
        .sort(doc! { "created_at": 1 })
        .await
        .map_err(|e| format!("Find error: {}", e))?;

    let mut comments = Vec::new();
    while let Some(doc) = cursor.try_next().await.map_err(|e| format!("Cursor error: {}", e))? {
        if let Some(comment) = doc_to_comment(&doc) {
            comments.push(comment);
        }
    }

    Ok(comments)
}

/// Get a single comment by ID.
pub async fn get_comment(id: &str) -> Result<Option<Comment>, String> {
    let col = get_comments_collection();
    let oid = ObjectId::parse_str(id).map_err(|e| format!("Invalid id: {}", e))?;

    let doc = col
        .find_one(doc! { "_id": oid })
        .await
        .map_err(|e| format!("Find error: {}", e))?;

    Ok(doc.as_ref().and_then(|d| doc_to_comment(d)))
}

/// Update a comment's content (re-runs sentiment analysis).
pub async fn update_comment(id: &str, new_content: &str) -> Result<Comment, String> {
    let col = get_comments_collection();
    let oid = ObjectId::parse_str(id).map_err(|e| format!("Invalid id: {}", e))?;
    let now = bson::DateTime::now();

    let sentiment_status = analyze_sentiment(new_content);
    let scoring = compute_scoring(new_content);

    col.update_one(
        doc! { "_id": oid },
        doc! {
            "$set": {
                "content": new_content,
                "status": sentiment_status as i32,
                "scoring": scoring as i32,
                "updated_at": now,
            }
        },
    )
    .await
    .map_err(|e| format!("Update error: {}", e))?;

    get_comment(id)
        .await?
        .ok_or_else(|| "Comment not found after update".to_string())
}

/// Like a comment (increment likes_count).
pub async fn like_comment(id: &str) -> Result<Comment, String> {
    let col = get_comments_collection();
    let oid = ObjectId::parse_str(id).map_err(|e| format!("Invalid id: {}", e))?;

    col.update_one(
        doc! { "_id": oid },
        doc! {
            "$inc": { "likes_count": 1 },
            "$set": { "updated_at": bson::DateTime::now() },
        },
    )
    .await
    .map_err(|e| format!("Like error: {}", e))?;

    get_comment(id)
        .await?
        .ok_or_else(|| "Comment not found after like".to_string())
}

/// Soft-delete a comment.
pub async fn soft_delete_comment(id: &str) -> Result<(), String> {
    let col = get_comments_collection();
    let oid = ObjectId::parse_str(id).map_err(|e| format!("Invalid id: {}", e))?;

    col.update_one(
        doc! { "_id": oid },
        doc! {
            "$set": {
                "is_deleted": true,
                "updated_at": bson::DateTime::now(),
            }
        },
    )
    .await
    .map_err(|e| format!("Delete error: {}", e))?;

    Ok(())
}

/// Get all comments (for admin/debug purposes), including deleted.
pub async fn get_all_comments() -> Result<Vec<Comment>, String> {
    let col = get_comments_collection();

    let mut cursor = col
        .find(doc! {})
        .sort(doc! { "created_at": -1 })
        .await
        .map_err(|e| format!("Find error: {}", e))?;

    let mut comments = Vec::new();
    while let Some(doc) = cursor.try_next().await.map_err(|e| format!("Cursor error: {}", e))? {
        if let Some(comment) = doc_to_comment(&doc) {
            comments.push(comment);
        }
    }

    Ok(comments)
}

/// Get negative comments for a post.
pub async fn get_negative_comments(post_id: &str) -> Result<Vec<Comment>, String> {
    let col = get_comments_collection();
    // Use string-based post_id matching

    let filter = doc! {
        "post_id": post_id,
        "status": 1_i32,
        "is_deleted": false,
    };

    let mut cursor = col
        .find(filter)
        .sort(doc! { "created_at": -1 })
        .await
        .map_err(|e| format!("Find error: {}", e))?;

    let mut comments = Vec::new();
    while let Some(doc) = cursor.try_next().await.map_err(|e| format!("Cursor error: {}", e))? {
        if let Some(comment) = doc_to_comment(&doc) {
            comments.push(comment);
        }
    }

    Ok(comments)
}
