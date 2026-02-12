use leptos::prelude::*;
use crate::model::Comment;

#[component]
pub fn CommentCard(
    comment: Comment,
    on_like: Callback<String>,
    on_delete: Callback<String>,
    current_user: ReadSignal<String>,
) -> impl IntoView {
    let comment_id = comment.id.clone().unwrap_or_default();
    let comment_id_like = comment_id.clone();
    let comment_id_delete = comment_id.clone();

    let sentiment_class = comment.sentiment_css_class().to_string();
    let sentiment_label = comment.sentiment_label().to_string();

    let author_short = if comment.author_public_key.len() > 12 {
        format!(
            "{}...{}",
            &comment.author_public_key[..6],
            &comment.author_public_key[comment.author_public_key.len()-4..]
        )
    } else {
        comment.author_public_key.clone()
    };

    let author_key = comment.author_public_key.clone();
    let is_own_comment = move || current_user.get() == author_key;

    let depth_class = format!("comment-depth-{}", comment.depth.min(4));
    let time_ago = format_time_ago(comment.created_at);

    view! {
        <div class={format!("comment-card {} {}", sentiment_class, depth_class)}>
            <div class="comment-header">
                <div class="comment-author">
                    <span class="author-avatar">"👤"</span>
                    <span class="author-key">{author_short}</span>
                </div>
                <div class="comment-meta">
                    <span class={format!("sentiment-badge sentiment-{}", sentiment_label)}>
                        {sentiment_label.clone()}
                    </span>
                    <span class="comment-time">{time_ago}</span>
                </div>
            </div>

            <div class="comment-body">
                <p class="comment-content">{comment.content.clone()}</p>
            </div>

            <div class="comment-actions">
                <button
                    class="action-btn like-btn"
                    on:click=move |_| on_like.run(comment_id_like.clone())
                >
                    <span class="action-icon">"❤"</span>
                    <span class="action-count">{comment.likes_count}</span>
                </button>

                {
                    let is_own = is_own_comment.clone();
                    let cid = comment_id_delete.clone();
                    move || {
                        if is_own() {
                            let cid = cid.clone();
                            Some(view! {
                                <button
                                    class="action-btn delete-btn"
                                    on:click=move |_| on_delete.run(cid.clone())
                                >
                                    <span class="action-icon">"🗑"</span>
                                    " Delete"
                                </button>
                            })
                        } else {
                            None
                        }
                    }
                }

                <div class="comment-score">
                    <span class="score-label">"Score:"</span>
                    <span class="score-value">{comment.scoring}</span>
                </div>
            </div>
        </div>
    }
}

/// Format a timestamp (millis since epoch) into a human-readable "time ago" string.
fn format_time_ago(millis: i64) -> String {
    let now = js_sys::Date::now() as i64;
    let diff_secs = (now - millis) / 1000;

    if diff_secs < 60 {
        "just now".to_string()
    } else if diff_secs < 3600 {
        format!("{}m ago", diff_secs / 60)
    } else if diff_secs < 86400 {
        format!("{}h ago", diff_secs / 3600)
    } else {
        format!("{}d ago", diff_secs / 86400)
    }
}
