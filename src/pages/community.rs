use leptos::prelude::*;
use crate::model::Comment;
use crate::api::*;
use crate::components::comment_card::CommentCard;
use crate::components::comment_form::CommentForm;

#[component]
pub fn Community(
    wallet_public_key: ReadSignal<String>,
) -> impl IntoView {
    // Fixed community post ID — stable across page loads so comments persist
    // Fixed community post ID — stable across page loads so comments persist
    let (post_id, _set_post_id) = signal("community-main".to_string());

    // Resource for fetching comments (SSR compatible)
    let comments_res = Resource::new(
        move || post_id.get(),
        move |pid| get_comments_by_post(pid)
    );

    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (filter, set_filter) = signal("all".to_string());

    // Submit a new comment
    let on_submit = move |pid: String, author: String, content: String| {
        leptos::task::spawn_local(async move {
            match create_comment(pid.clone(), author.clone(), content.clone(), None, 0).await {
                Ok(comment) => {
                    // Refetch
                    comments_res.refetch();
                    
                    if comment.status == 1 {
                        web_sys::console::log_1(&"Negative comment detected...".into());
                        match get_soroban_config().await {
                             Ok(config) => {
                                 let content_hash = if content.len() > 32 { content[..32].to_string() + "..." } else { content.clone() };
                                 match crate::components::wallet::submit_negative_comment_client(
                                     config.rpc_url, config.network_passphrase, config.contract_id,
                                     comment.id.clone().unwrap_or_default(), comment.scoring as u32, content_hash, author
                                 ).await {
                                     Ok(res) => {
                                         if res.starts_with("SUCCESS:") {
                                             let hash = res.trim_start_matches("SUCCESS:");
                                             web_sys::console::log_1(&format!("✅ Soroban TX Success: {}", hash).into());
                                         } else {
                                             web_sys::console::error_1(&format!("❌ Soroban TX Failed: {}", res).into());
                                             set_error_msg.set(Some(format!("Submission Error: {}", res)));
                                         }
                                     },
                                     Err(e) => web_sys::console::error_1(&format!("TX Exception: {}", e).into())
                                 }
                             }
                             Err(e) => web_sys::console::error_1(&format!("Config error: {}", e).into())
                        }
                    }
                }
                Err(e) => set_error_msg.set(Some(format!("Failed to post: {}", e)))
            }
        });
    };

    // Like callback
    let on_like = Callback::new(move |id: String| {
        leptos::task::spawn_local(async move {
            match like_comment(id.clone()).await {
                Ok(_) => comments_res.refetch(),
                Err(e) => web_sys::console::log_1(&format!("Error liking: {}", e).into())
            }
        });
    });

    // Delete callback
    let on_delete = Callback::new(move |id: String| {
        leptos::task::spawn_local(async move {
            match delete_comment(id).await {
                Ok(_) => comments_res.refetch(),
                Err(e) => web_sys::console::log_1(&format!("Error deleting: {}", e).into())
            }
        });
    });

    // Derived signals from resource
    let comments_data = move || comments_res.get()
        .and_then(|r| r.ok())
        .unwrap_or_default();
    
    let filtered_comments = move || {
        let current_filter = filter.get();
        let all = comments_data();
        match current_filter.as_str() {
            "negative" => all.iter().filter(|c| c.status == 1).cloned().collect(),
            "neutral" => all.iter().filter(|c| c.status == 2).cloned().collect(),
            "positive" => all.iter().filter(|c| c.status == 3).cloned().collect(),
            _ => all,
        }
    };

    let comment_count = move || comments_data().len();
    let neg_count = move || comments_data().iter().filter(|c| c.status == 1).count();
    let neu_count = move || comments_data().iter().filter(|c| c.status == 2).count();
    let pos_count = move || comments_data().iter().filter(|c| c.status == 3).count();

    view! {
        <div class="community-page">
            <div class="community-container">
                <div class="community-header">
                    <h1 class="community-title">"Community Reviews"</h1>
                    <p class="community-subtitle">"Real experiences from our valued guests"</p>
                </div>

                <Suspense fallback=move || view! { <div class="loading-spinner"></div> }>
                    <div class="stats-bar">
                        <div class="stat-item">
                            <span class="stat-number">{comment_count}</span>
                            <span class="stat-label">"Reviews"</span>
                        </div>
                        <div class="stat-item stat-negative">
                            <span class="stat-number">{neg_count}</span>
                            <span class="stat-label">"Negative"</span>
                        </div>
                        <div class="stat-item stat-neutral">
                            <span class="stat-number">{neu_count}</span>
                            <span class="stat-label">"Neutral"</span>
                        </div>
                        <div class="stat-item stat-positive">
                            <span class="stat-number">{pos_count}</span>
                            <span class="stat-label">"Positive"</span>
                        </div>
                    </div>

                    <div class="filter-tabs">
                         <button class="filter-tab" class:active=move || filter.get() == "all" on:click=move |_| set_filter.set("all".to_string())>"All"</button>
                         <button class="filter-tab filter-tab-negative" class:active=move || filter.get() == "negative" on:click=move |_| set_filter.set("negative".to_string())>"Negative"</button>
                         <button class="filter-tab filter-tab-neutral" class:active=move || filter.get() == "neutral" on:click=move |_| set_filter.set("neutral".to_string())>"Neutral"</button>
                         <button class="filter-tab filter-tab-positive" class:active=move || filter.get() == "positive" on:click=move |_| set_filter.set("positive".to_string())>"Positive"</button>
                    </div>

                    <CommentForm
                        post_id=post_id
                        author_key=wallet_public_key
                        on_submit=on_submit
                    />
                    
                    <Show when=move || error_msg.get().is_some()>
                        <div class="error-banner">{move || error_msg.get().unwrap_or_default()}</div>
                    </Show>

                    <div class="comments-list">
                        <For
                            each=filtered_comments
                            key=|comment| comment.id.clone().unwrap_or_default()
                            let:comment
                        >
                            <CommentCard
                                comment=comment
                                on_like=on_like
                                on_delete=on_delete
                                current_user=wallet_public_key
                            />
                        </For>
                        <Show when=move || filtered_comments().is_empty()>
                             <div class="empty-state"><p>"No reviews yet."</p></div>
                        </Show>
                    </div>
                </Suspense>
            </div>
        </div>
    }
}
