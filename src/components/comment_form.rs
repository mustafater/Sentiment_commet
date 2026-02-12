use leptos::prelude::*;

#[component]
pub fn CommentForm(
    post_id: ReadSignal<String>,
    author_key: ReadSignal<String>,
    on_submit: impl Fn(String, String, String) + 'static + Send + Sync + Clone,
) -> impl IntoView {
    let (content, set_content) = signal(String::new());
    let (submitting, set_submitting) = signal(false);

    let is_connected = move || !author_key.get().is_empty();

    let on_submit_clone = on_submit.clone();
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let text = content.get();
        if text.trim().is_empty() { return; }

        set_submitting.set(true);
        let pid = post_id.get();
        let key = author_key.get();
        on_submit_clone(pid, key, text);
        set_content.set(String::new());
        set_submitting.set(false);
    };

    view! {
        <div class="comment-form-container">
            <h3 class="form-title">"Share Your Experience"</h3>
            {
                let is_conn = is_connected.clone();
                let handle = handle_submit.clone();
                move || {
                    if is_conn() {
                        let h = handle.clone();
                        Some(view! {
                            <form class="comment-form" on:submit=h>
                                <div class="form-group">
                                    <textarea
                                        class="comment-textarea"
                                        placeholder="Share your dining experience..."
                                        prop:value=content
                                        on:input=move |ev| {
                                            set_content.set(event_target_value(&ev));
                                        }
                                        rows=4
                                        maxlength=500
                                    ></textarea>
                                    <div class="char-count">
                                        {move || format!("{}/500", content.get().len())}
                                    </div>
                                </div>
                                <button
                                    type="submit"
                                    class="submit-btn"
                                    disabled=move || submitting.get() || content.get().trim().is_empty()
                                >
                                    {move || if submitting.get() { "Posting..." } else { "Post Review" }}
                                </button>
                            </form>
                        })
                    } else {
                        None
                    }
                }
            }
            <Show when=move || !is_connected()>
                <div class="connect-prompt">
                    <p class="prompt-text">"Connect your Freighter wallet to leave a review"</p>
                    <div class="prompt-icon">"🔐"</div>
                </div>
            </Show>
        </div>
    }
}
