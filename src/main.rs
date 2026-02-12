#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use dene::app::App;

    // Initialize logger
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("Failed to init logger");

    // Initialize MongoDB (graceful — app starts even if DB is down)
    match dene::server::db::init_db().await {
        Ok(()) => log::info!("✅ MongoDB initialized"),
        Err(e) => log::warn!("⚠️  MongoDB init error (app will start, DB ops may fail): {}", e),
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let site_root = leptos_options.site_root.clone();

    log::info!("🚀 Server starting at http://{}", addr);

    let routes = generate_route_list(App);

    HttpServer::new(move || {
        let leptos_options = leptos_options.clone();
        let site_root_str = site_root.to_string();

        actix_web::App::new()
            // Static files MUST come before leptos_routes so /pkg/* is served correctly
            .service(Files::new("/pkg", format!("{}/pkg", site_root_str)))
            .service(Files::new("/assets", format!("{}", site_root_str)))
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                move || {
                    use leptos::prelude::*;
                    use leptos_meta::MetaTags;

                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone()/>
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                                <link rel="stylesheet" id="leptos" href="/pkg/dene.css"/>
                                <link rel="icon" type="image/x-icon" href="/favicon.ico"/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .service(Files::new("/", site_root_str))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Client-side only, hydration is handled by lib.rs
}
