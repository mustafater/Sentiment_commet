use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="dashboard-page">
            // Hero Section
            <section class="hero">
                <div class="hero-overlay"></div>
                <div class="hero-content">
                    <span class="hero-badge">"✦ Fine Dining Experience ✦"</span>
                    <h1 class="hero-title">"Maison Dorée"</h1>
                    <p class="hero-subtitle">"Where culinary artistry meets timeless elegance"</p>
                    <div class="hero-actions">
                        <a href="/community" class="hero-btn hero-btn-primary">"View Community"</a>
                        <a href="#menu" class="hero-btn hero-btn-secondary">"Our Menu"</a>
                    </div>
                </div>
            </section>

            // Menu Section
            <section id="menu" class="menu-section">
                <div class="section-container">
                    <div class="section-header">
                        <span class="section-badge">"Our Specialties"</span>
                        <h2 class="section-title">"Curated Menu"</h2>
                        <p class="section-description">"Each dish is a masterpiece, crafted with passion and precision"</p>
                    </div>

                    <div class="menu-grid">
                        <div class="menu-card">
                            <div class="menu-card-icon">"🥩"</div>
                            <h3 class="menu-card-title">"Wagyu Tenderloin"</h3>
                            <p class="menu-card-desc">"A5 Japanese Wagyu, truffle jus, roasted bone marrow"</p>
                            <span class="menu-card-price">"$120"</span>
                        </div>

                        <div class="menu-card">
                            <div class="menu-card-icon">"🦞"</div>
                            <h3 class="menu-card-title">"Butter-Poached Lobster"</h3>
                            <p class="menu-card-desc">"Maine lobster, saffron beurre blanc, micro herbs"</p>
                            <span class="menu-card-price">"$95"</span>
                        </div>

                        <div class="menu-card">
                            <div class="menu-card-icon">"🍝"</div>
                            <h3 class="menu-card-title">"Black Truffle Risotto"</h3>
                            <p class="menu-card-desc">"Carnaroli rice, Périgord truffle, aged Parmesan"</p>
                            <span class="menu-card-price">"$78"</span>
                        </div>

                        <div class="menu-card">
                            <div class="menu-card-icon">"🐟"</div>
                            <h3 class="menu-card-title">"Pan-Seared Sea Bass"</h3>
                            <p class="menu-card-desc">"Chilean sea bass, miso glaze, edamame purée"</p>
                            <span class="menu-card-price">"$88"</span>
                        </div>

                        <div class="menu-card">
                            <div class="menu-card-icon">"🥗"</div>
                            <h3 class="menu-card-title">"Garden of Eden Salad"</h3>
                            <p class="menu-card-desc">"Heirloom tomatoes, burrata, basil oil, balsamic"</p>
                            <span class="menu-card-price">"$32"</span>
                        </div>

                        <div class="menu-card">
                            <div class="menu-card-icon">"🍰"</div>
                            <h3 class="menu-card-title">"Crème Brûlée Trio"</h3>
                            <p class="menu-card-desc">"Vanilla, lavender, and passion fruit variations"</p>
                            <span class="menu-card-price">"$28"</span>
                        </div>
                    </div>
                </div>
            </section>

            // Features Section
            <section class="features-section">
                <div class="section-container">
                    <div class="features-grid">
                        <div class="feature-card">
                            <div class="feature-icon">"⛓"</div>
                            <h3 class="feature-title">"On-Chain Reviews"</h3>
                            <p class="feature-desc">"Community feedback stored transparently on Stellar blockchain"</p>
                        </div>
                        <div class="feature-card">
                            <div class="feature-icon">"☕"</div>
                            <h3 class="feature-title">"Earn Beans"</h3>
                            <p class="feature-desc">"Get rewarded with Beans tokens for quality reviews"</p>
                        </div>
                        <div class="feature-card">
                            <div class="feature-icon">"🤖"</div>
                            <h3 class="feature-title">"AI Sentiment"</h3>
                            <p class="feature-desc">"Automatic sentiment analysis on every review"</p>
                        </div>
                    </div>
                </div>
            </section>

            // Footer
            <footer class="footer">
                <div class="section-container">
                    <div class="footer-content">
                        <div class="footer-brand">
                            <span class="footer-logo">"🍽"</span>
                            <span class="footer-name">"Maison Dorée"</span>
                        </div>
                        <p class="footer-text">"Powered by Stellar • Built with Leptos"</p>
                        <p class="footer-copyright">"© 2026 Maison Dorée. All rights reserved."</p>
                    </div>
                </div>
            </footer>
        </div>
    }
}
