use leptos::prelude::*;
use crate::model::Comment;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SorobanConfig {
    pub contract_id: String,
    pub network_passphrase: String,
    pub rpc_url: String,
}

#[server(GetSorobanConfig, "/api")]
pub async fn get_soroban_config() -> Result<SorobanConfig, ServerFnError> {
    // Load local environment if needed, though main.rs handles this
    // dotenv::dotenv().ok(); 
    
    let contract_id = std::env::var("CONTRACT_ID").unwrap_or_default();
    // Default to testnet if not specified
    let network = std::env::var("NETWORK").unwrap_or_else(|_| "testnet".to_string());
    
    let (passphrase, rpc) = if network == "futurenet" {
         ("Test SDF Future Network ; October 2022", "https://rpc-futurenet.stellar.org")
    } else if network == "mainnet" {
        ("Public Global Stellar Network ; September 2015", "https://horizon.stellar.org")
    } else {
        // Default testnet
        ("Test SDF Network ; September 2015", "https://soroban-testnet.stellar.org")
    };

    Ok(SorobanConfig {
        contract_id,
        network_passphrase: passphrase.to_string(),
        rpc_url: rpc.to_string(),
    })
}

/// Create a new comment. Sentiment analysis is performed automatically.
#[server(CreateComment, "/api")]
pub async fn create_comment(
    post_id: String,
    author_public_key: String,
    content: String,
    parent_id: Option<String>,
    depth: u8,
) -> Result<Comment, ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::create_comment(&post_id, &author_public_key, &content, parent_id, depth)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Get all comments for a post.
#[server(GetCommentsByPost, "/api")]
pub async fn get_comments_by_post(post_id: String) -> Result<Vec<Comment>, ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::get_comments_by_post(&post_id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Get a single comment by ID.
#[server(GetComment, "/api")]
pub async fn get_comment(id: String) -> Result<Option<Comment>, ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::get_comment(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Update a comment's content.
#[server(UpdateComment, "/api")]
pub async fn update_comment(id: String, content: String) -> Result<Comment, ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::update_comment(&id, &content)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Like a comment.
#[server(LikeComment, "/api")]
pub async fn like_comment(id: String) -> Result<Comment, ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::like_comment(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Soft-delete a comment.
#[server(DeleteComment, "/api")]
pub async fn delete_comment(id: String) -> Result<(), ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::soft_delete_comment(&id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Get all comments (admin).
#[server(GetAllComments, "/api")]
pub async fn get_all_comments() -> Result<Vec<Comment>, ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::get_all_comments()
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Get negative comments for a post.
#[server(GetNegativeComments, "/api")]
pub async fn get_negative_comments(post_id: String) -> Result<Vec<Comment>, ServerFnError> {
    use crate::server::comment_crud;

    comment_crud::get_negative_comments(&post_id)
        .await
        .map_err(|e| ServerFnError::new(e))
}

/// Create a demo post_id for testing (returns a valid ObjectId string).
#[server(CreateDemoPost, "/api")]
pub async fn create_demo_post() -> Result<String, ServerFnError> {
    use bson::oid::ObjectId;
    Ok(ObjectId::new().to_hex())
}
