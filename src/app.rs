use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::components::navbar::Navbar;
use crate::components::wallet::WalletState;
use crate::pages::dashboard::Dashboard;
use crate::pages::community::Community;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (wallet_state, set_wallet_state) = signal(WalletState::default());
    let (wallet_pk, set_wallet_pk) = signal(String::new());

    // Sync wallet public key to a simple string signal
    Effect::new(move |_| {
        let key = wallet_state.get().public_key.clone();
        set_wallet_pk.set(key);
    });

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light"/>
        <Title text="Maison Dorée — Fine Dining & Community"/>
        <Meta name="description" content="Maison Dorée: Where culinary artistry meets blockchain-powered community reviews on Stellar"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" attr:crossorigin="anonymous"/>
        <Link href="https://fonts.googleapis.com/css2?family=Playfair+Display:wght@400;500;600;700&family=Inter:wght@300;400;500;600&display=swap" rel="stylesheet"/>
        
        <Navbar wallet_state=wallet_state set_wallet_state=set_wallet_state/>

        <main class="main-content">
            <Router>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=Dashboard/>
                    <Route path=path!("/community") view=move || {
                        view! { <Community wallet_public_key=wallet_pk/> }
                    }/>
                </Routes>
            </Router>
        </main>
    }
}
