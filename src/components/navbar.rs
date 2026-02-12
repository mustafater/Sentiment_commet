use leptos::prelude::*;
use crate::components::wallet::{WalletState, WalletButton};

#[component]
pub fn Navbar(
    wallet_state: ReadSignal<WalletState>,
    set_wallet_state: WriteSignal<WalletState>,
) -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    let wallet_display = move || {
        let state = wallet_state.get();
        if state.connected {
            let key = state.public_key.clone();
            if key.len() > 10 {
                format!("{}...{}", &key[..4], &key[key.len()-4..])
            } else {
                key
            }
        } else {
            String::new()
        }
    };

    let beans_display = move || {
        let state = wallet_state.get();
        if state.connected {
            format!("{} Beans", state.beans_balance)
        } else {
            String::new()
        }
    };

    view! {
        <nav class="navbar">
            <div class="navbar-container">
                // Left: Logo & Restaurant Name
                <div class="navbar-left">
                    <div class="navbar-logo">
                        <span class="logo-icon">"🍽"</span>
                    </div>
                    <a href="/" class="navbar-brand">"Maison Dorée"</a>
                </div>

                // Mobile menu toggle
                <button
                    class="mobile-menu-toggle"
                    on:click=move |_| set_mobile_menu_open.update(|v| *v = !*v)
                >
                    <span class="hamburger-line"></span>
                    <span class="hamburger-line"></span>
                    <span class="hamburger-line"></span>
                </button>

                // Middle: Navigation
                <div class="navbar-center" class:mobile-open=mobile_menu_open>
                    <a href="/" class="nav-link">"Menu"</a>
                    <a href="/community" class="nav-link">"Community"</a>
                </div>

                // Right: Wallet & Network
                <div class="navbar-right" class:mobile-open=mobile_menu_open>
                    {move || {
                        let state = wallet_state.get();
                        if state.connected {
                            Some(view! {
                                <div class="beans-badge">
                                    <span class="beans-icon">"☕"</span>
                                    <span class="beans-count">{beans_display}</span>
                                </div>
                            })
                        } else {
                            None
                        }
                    }}

                    <div class="network-badge">
                        <span class="network-dot"></span>
                        <span class="network-name">"Testnet"</span>
                    </div>

                    <WalletButton wallet_state=wallet_state set_wallet_state=set_wallet_state/>
                </div>
            </div>
        </nav>
    }
}
