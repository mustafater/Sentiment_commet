use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Wallet state shared across the app.
#[derive(Clone, Debug, Default)]
pub struct WalletState {
    pub connected: bool,
    pub public_key: String,
    pub beans_balance: u64,
}

// Import JS from external file
#[wasm_bindgen(module = "/src/wallet_interop.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn freighter_connect() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn freighter_get_public_key() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn freighter_is_connected() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn freighter_sign_tx(xdr: &str, network: &str) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(catch)]
    async fn submit_negative_comment_js(
        rpcUrl: &str,
        networkPassphrase: &str,
        contractId: &str,
        commentId: &str,
        score: u32,
        contentHash: &str,
        publicKey: &str
    ) -> Result<JsValue, JsValue>;
}

pub async fn connect_wallet() -> Result<String, String> {
    let result = freighter_connect().await.map_err(|e| format!("{:?}", e))?;
    let key = result.as_string().unwrap_or_default();
    if key.is_empty() { Err("Freighter returned empty key".into()) } else { Ok(key) }
}

pub async fn is_wallet_connected() -> bool {
    match freighter_is_connected().await {
        Ok(v) => v.as_bool().unwrap_or(false),
        Err(_) => false,
    }
}

pub async fn get_public_key() -> Result<String, String> {
    freighter_get_public_key().await.map_err(|e| format!("{:?}", e))?
        .as_string().ok_or("No key".into())
}

pub async fn sign_transaction(xdr: &str) -> Result<String, String> {
    freighter_sign_tx(xdr, "TESTNET").await.map_err(|e| format!("{:?}", e))?
        .as_string().ok_or("No signed XDR".into())
}

pub async fn submit_negative_comment_client(
    rpc_url: String,
    network_passphrase: String,
    contract_id: String,
    comment_id: String,
    score: u32,
    content_hash: String,
    public_key: String,
) -> Result<String, String> {
    submit_negative_comment_js(
        &rpc_url, &network_passphrase, &contract_id, &comment_id, score, &content_hash, &public_key
    ).await.map_err(|e| format!("{:?}", e))?
    .as_string().ok_or("No result".into())
}

#[component]
pub fn WalletButton(
    wallet_state: ReadSignal<WalletState>,
    set_wallet_state: WriteSignal<WalletState>,
) -> impl IntoView {
    let (connecting, set_connecting) = signal(false);

    let do_connect = move |_: web_sys::MouseEvent| {
        if connecting.get_untracked() { return; }
        set_connecting.set(true);
        leptos::task::spawn_local(async move {
            match connect_wallet().await {
                Ok(key) => {
                    set_wallet_state.set(WalletState {
                        connected: true,
                        public_key: key,
                        beans_balance: 1250,
                    });
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Wallet error: {}", e).into());
                }
            }
            set_connecting.set(false);
        });
    };

    let do_disconnect = move |_: web_sys::MouseEvent| {
        set_wallet_state.set(WalletState::default());
    };

    view! {
        <div class="wallet-component">
            {move || {
                let state = wallet_state.get();
                if state.connected {
                    let key = state.public_key.clone();
                    let display_key = if key.len() > 12 {
                        format!("{}...{}", &key[..6], &key[key.len()-6..])
                    } else { key };
                    leptos::either::Either::Left(view! {
                        <div class="wallet-info">
                            <span class="wallet-address-full">{display_key}</span>
                            <button class="wallet-btn wallet-btn-disconnect" on:click=do_disconnect>"Disconnect"</button>
                        </div>
                    })
                } else {
                    let conn = connecting.get();
                    leptos::either::Either::Right(view! {
                        <button class="wallet-btn wallet-btn-connect" on:click=do_connect.clone()>
                            <span class="wallet-icon">"🔗"</span>
                            {if conn { " Connecting..." } else { " Connect Freighter" }}
                        </button>
                    })
                }
            }}
        </div>
    }
}
